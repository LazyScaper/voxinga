use avian3d::prelude::{AngularVelocity, Collider, LockedAxes, PhysicsDebugPlugin, RigidBody};
use avian3d::PhysicsPlugins;
use bevy::color::palettes::css;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorOptions, PresentMode};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera {
    pitch: f32,
    sensitivity: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            sensitivity: 0.002,
        }
    }
}

#[derive(Component)]
struct PlayerYaw {
    yaw: f32,
}

impl Default for PlayerYaw {
    fn default() -> Self {
        Self { yaw: 0.0 }
    }
}

fn toggle_cursor_lock(
    mut cursor_options: Single<&mut CursorOptions>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match cursor_options.grab_mode {
            bevy::window::CursorGrabMode::None => {
                cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
                cursor_options.visible = false;
            }
            _ => {
                cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
                cursor_options.visible = true;
            }
        }
    }
}

fn camera_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut player_query: Query<(&mut Transform, &mut PlayerYaw), With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &mut PlayerCamera),
        (With<PlayerCamera>, Without<Player>),
    >,
) {
    let Ok((mut player_transform, mut player_yaw)) = player_query.single_mut() else {
        return;
    };

    let Ok((mut camera_transform, mut camera)) = camera_query.single_mut() else {
        return;
    };

    for motion in mouse_motion.read() {
        player_yaw.yaw -= motion.delta.x * camera.sensitivity;
        camera.pitch -= motion.delta.y * camera.sensitivity;

        // Clamp pitch to prevent flipping
        camera.pitch = camera.pitch.clamp(-1.54, 1.54);
    }

    // Apply yaw to player body (rotation around Y axis)
    player_transform.rotation = Quat::from_rotation_y(player_yaw.yaw);

    // Apply pitch to camera (rotation around X axis)
    camera_transform.rotation = Quat::from_rotation_x(camera.pitch);
}
fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut controller_query: Query<&mut TnuaController>,
) {
    let Ok(mut controller) = controller_query.single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction += player_transform.forward().as_vec3();
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += player_transform.back().as_vec3();
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction += player_transform.left().as_vec3();
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += player_transform.right().as_vec3();
    }

    // Flatten direction to XZ plane for walking
    direction.y = 0.0;

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction.normalize_or_zero() * 1.1,
        float_height: 1.5,
        ..Default::default()
    });

    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: 4.0,
            ..Default::default()
        });
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    setup_gui(&mut commands);
    setup_grid(&mut commands, &mut meshes, &mut materials);
    setup_player(&mut commands, &mut meshes, &mut materials);

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 10.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(2.0, 4.0, 2.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn setup_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let grid_size = 10;
    for i in 0..grid_size {
        for j in 0..grid_size {
            let color = if (i + j) % 2 == 0 {
                Color::srgb(0.0, 0.0, 0.0) // Green
            } else {
                Color::srgb(255.0, 255.0, 255.0) // Red
            };

            commands.spawn((
                RigidBody::Static,
                Mesh3d(meshes.add(Cuboid::from_length(1f32))),
                MeshMaterial3d(materials.add(color)),
                Collider::cuboid(1.0, 1.0, 1.0),
                Transform::from_xyz(i as f32, 0f32, j as f32),
            ));
        }
    }
}

fn setup_gui(commands: &mut Commands) {
    commands.spawn((
        Text::new("FPS: "),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        FpsText,
    ));
}

fn setup_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d {
                radius: 0.5,
                half_length: 0.5,
            })),
            MeshMaterial3d(materials.add(Color::from(css::DARK_CYAN))),
            Transform::from_xyz(3.0, 2.0, 3.0),
            RigidBody::Dynamic,
            Collider::capsule(0.5, 1.0),
            TnuaController::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
            LockedAxes::ROTATION_LOCKED,
            Player,
            PlayerYaw::default(),
        ))
        .with_children(|parent| {
            // Spawn camera as a child of the player
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.5, 0.0),
                PlayerCamera::default(),
            ));
        });
}

fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        let fps = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
            .unwrap_or(-1.0);

        let frame_time = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .and_then(|time| time.smoothed())
            .unwrap_or(-1.0);

        **text = format!("FPS: {:.2} ({:.2}ms)", fps, frame_time);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                fps_update_system,
                apply_controls.in_set(TnuaUserControlsSystems),
            ),
        )
        .add_systems(Update, (camera_look, toggle_cursor_lock))
        .run();
}
