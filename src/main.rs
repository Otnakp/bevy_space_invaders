use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode, WindowResized};
const SPRITE_SHIFT: f32 = 100.0;
const SPRITE_SIZE: f32 = 100.0;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space invaders 2".into(),
                        resizable: true,
                        mode: WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, on_resize_system))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());
    let window = windows.get_single().unwrap();
    let half_height = window.height() / 2.0;
    let texture = asset_server.load("character.png");
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
            ..default()
        },
        texture,
        transform: Transform::from_xyz(0.0, -half_height + SPRITE_SHIFT, 0.0),
        ..default()
    });
}

fn player_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::D) {
            transform.translation.x += 200.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 200.0 * time.delta_seconds();
        }
    }
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<&mut Transform, &Sprite>,
) {
    for e in resize_reader.iter() {
        let height = e.height;
        let half_height = height / 2.0;
        for mut transform in query.iter_mut() {
            transform.translation.y = -half_height + SPRITE_SHIFT;
        }
    }
}
