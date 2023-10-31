use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::systems::constants::*;
use crate::systems::projectile::*;
use crate::systems::text::*;

use super::enemy::Enemy;
use bevy::window::PrimaryWindow;

#[derive(Component, Deref, DerefMut)]
pub struct ShootTimer(pub Timer);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite: SpriteBundle,
}
pub fn player_movement(
    mut character: Query<(&mut Transform, &Sprite), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, _) = character.single_mut();
    if input.pressed(KeyCode::D) {
        transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
}
pub fn move_projectile(
    mut query: Query<(&mut Transform, &Sprite), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut transform, _sprite) in query.iter_mut() {
        transform.translation.y += 200.0 * time.delta_seconds();
    }
}

pub fn check_collision(
    mut enemies: Query<(Entity, &mut Transform, &Sprite), (With<Enemy>, Without<Projectile>)>,
    mut projectiles: Query<(Entity, &mut Transform, &Sprite), (With<Projectile>, Without<Player>)>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut texts: Query<&mut Text, With<PointText>>,
) {
    let window = windows.get_single().unwrap();
    let half_height = window.height() / 2.0;

    let mut to_despawn = HashMap::new();
    for (enemy_entity, enemy_transform, _) in enemies.iter_mut() {
        for (projectile_entity, projectile_transform, _) in projectiles.iter_mut() {
            let half_enemy_size = ENEMY_SIZE / 2.0;
            let half_projectile_size = PROJECTILE_SIZE / 2.0;
            let enemy_left = enemy_transform.translation.x - half_enemy_size;
            let enemy_right = enemy_transform.translation.x + half_enemy_size;
            let enemy_top = enemy_transform.translation.y + half_enemy_size;
            let enemy_bottom = enemy_transform.translation.y - half_enemy_size;

            let projectile_left = projectile_transform.translation.x - half_projectile_size;
            let projectile_right = projectile_transform.translation.x + half_projectile_size;
            let projectile_top = projectile_transform.translation.y + half_projectile_size;
            let projectile_bottom = projectile_transform.translation.y - half_projectile_size;

            let overlap_x = projectile_left < enemy_right && projectile_right > enemy_left;
            let overlap_y = projectile_top > enemy_bottom && projectile_bottom < enemy_top;

            if overlap_y && overlap_x {
                to_despawn.insert(enemy_entity, "Kill");
                to_despawn.insert(projectile_entity, "Projectile");
            }
            if projectile_bottom > half_height {
                to_despawn.insert(projectile_entity, "Projectile");
            }
        }
    }
    let mut text = texts.single_mut();
    let n = &text.sections[0].value;
    let mut n: i32 = n.parse().unwrap();
    for (entity, type_kill) in to_despawn.into_iter() {
        commands.entity(entity).despawn();
        //match entity {

        //}
        if type_kill == "Kill" {
            n += 1;
            text.sections[0].value = n.to_string();
        }
    }
}

pub fn shoot(
    mut character: Query<(&mut Transform, &Sprite), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer_query: Query<&mut ShootTimer>,
) {
    let mut timer = timer_query.single_mut();
    let mut finished = false;

    if timer.tick(time.delta()).finished() {
        finished = true;
    }
    if input.pressed(KeyCode::Space) && finished {
        timer.reset();
        let (transform, _) = character.single_mut();
        let shooting_pos = transform.translation;
        let texture = asset_server.load("projectile.png");
        commands.spawn(ProjectileBundle {
            projectile: Projectile,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PROJECTILE_SIZE, PROJECTILE_SIZE)),
                    ..default()
                },
                texture,
                transform: Transform::from_xyz(
                    shooting_pos.x,
                    shooting_pos.y + SPRITE_SIZE / 2.0,
                    0.0,
                ),
                ..default()
            },
        });
    }
}
