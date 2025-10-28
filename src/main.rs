use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::time::Duration;
use bevy::window::PresentMode;

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

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

        **text = format!("FPS: {:.2}", fps)
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
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(100)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, fps_update_system)
        .run();
}
