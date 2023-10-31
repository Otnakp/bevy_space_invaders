use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode, WindowResolution};
mod systems;
use systems::constants::*;
use systems::enemy::*;
use systems::player::*;
use systems::systems_impl::*;
use systems::text::*;
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
        .add_systems(Update, (shoot, move_projectile))
        .add_systems(Update, (spawn_enemy_when_completed, move_enemies))
        .add_systems(Update, check_collision)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(ShootTimer(Timer::from_seconds(
        SHOOT_TIMER,
        TimerMode::Once,
    )));
    commands.spawn(SpawnEnemyOnCompletionTimer(Timer::from_seconds(
        ENEMY_SPAWN_TIME,
        TimerMode::Repeating,
    )));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "0",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0), // Adjusted this line
            right: Val::Px(15.0),
            ..Default::default() // Note: Rust convention is to use Default::default() instead of default()
        }),
        PointText,
    ));

    commands.spawn(Camera2dBundle::default());
    let window = windows.get_single().unwrap();
    let half_height = window.height() / 2.0;
    let texture = asset_server.load("character.png");
    commands.spawn(PlayerBundle {
        player: Player,
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            texture,
            transform: Transform::from_xyz(0.0, -half_height + SPRITE_SHIFT, 0.0),
            ..default()
        },
    });
}
