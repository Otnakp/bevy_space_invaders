use bevy::prelude::*;
use bevy::window::PrimaryWindow;

extern crate rand;
use rand::Rng;

use crate::systems::constants::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub sprite: SpriteBundle,
}

#[derive(Component, Deref, DerefMut)]
pub struct SpawnEnemyOnCompletionTimer(pub Timer);

pub fn spawn_enemy_when_completed(
    time: Res<Time>,
    mut query: Query<&mut SpawnEnemyOnCompletionTimer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for mut timer in &mut query {
        let window = windows.get_single().unwrap();
        let spawn_y = window.height() / 2.0 - ENEMY_SIZE;
        let mut rng = rand::thread_rng();
        let random_x =
            rng.gen_range(-window.width() / 2.0 + ENEMY_SIZE..window.width() / 2.0 - ENEMY_SIZE);
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
pub fn move_enemies(
    mut query: Query<(&mut Transform, &Sprite), With<Enemy>>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let window = windows.get_single().unwrap();
    let border = -window.height() / 2.0 + SPRITE_SIZE;

    for (mut transform, _sprite) in query.iter_mut() {
        if transform.translation.y <= border {
            // println!("Game over");
        }
        transform.translation.y -= ENEMY_SPEED * time.delta_seconds();
    }
}
