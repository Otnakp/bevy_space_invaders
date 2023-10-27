use bevy::prelude::*;
use bevy::transform::commands;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode, WindowResolution};
extern crate rand;
use rand::Rng;
mod systems;
use systems::constants::*;
use systems::player::*;
use systems::systems_impl::*;
#[derive(Component)]
struct Enemy;
#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    sprite: SpriteBundle,
}
#[derive(Component, Deref, DerefMut)]
pub struct SpawnEnemyOnCompletionTimer(Timer);
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
        .add_systems(Update, spawn_enemy_when_completed)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(SpawnEnemyOnCompletionTimer(Timer::from_seconds(
        ENEMY_SPAWN_TIME,
        TimerMode::Repeating,
    )));
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
fn spawn_enemy_when_completed(
    time: Res<Time>,
    mut query: Query<&mut SpawnEnemyOnCompletionTimer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for mut timer in &mut query {
        let window = windows.get_single().unwrap();
        let spawn_y = window.height() / 2.0;
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(-window.width() / 2.0..window.width() / 2.0);
        if timer.tick(time.delta()).just_finished() {
            println!("Spawning enemy");
            let texture = asset_server.load("enemy.png");
            commands.spawn(EnemyBundle {
                enemy: Enemy,
                sprite: SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                        ..default()
                    },
                    texture,
                    transform: Transform::from_xyz(random_x, spawn_y, 0.0),
                    ..default()
                },
            });
        }
    }
}
