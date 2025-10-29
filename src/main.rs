use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    setup_gui(&mut commands);

    // cube
    for i in 1..10 {
        for j in 1..10 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(if (i + j) % 2 == 0 {
                    Color::BLACK
                } else {
                    Color::WHITE
                })),
                Transform::from_xyz(i as f32, 0.0, j as f32),
            ));
        }
    }

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera {
            yaw_upper_limit: Some(TAU / 4.0),
            yaw_lower_limit: Some(-TAU / 4.0),
            pitch_upper_limit: Some(TAU / 3.0),
            pitch_lower_limit: Some(-TAU / 3.0),
            zoom_upper_limit: Some(5.0),
            zoom_lower_limit: 1.0,
            // Adjust sensitivity of controls
            orbit_sensitivity: 0.15,
            pan_sensitivity: 0.05,
            zoom_sensitivity: 0.5,
            ..default()
        },
    ));
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
            PanOrbitCameraPlugin,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(100)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, fps_update_system)
        .run();
}
