use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode, WindowResolution};

mod systems;
use systems::constants::*;
use systems::systems_impl::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space invaders 2".into(),
                        resizable: false, // put true if you put borderless
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(500.0, 900.0),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, on_resize_system, check_borders))
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
