use bevy::prelude::*;

use crate::systems::constants::*;
use crate::systems::projectile::*;

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
        // destroy the eneml
        let texture = asset_server.load("projectile.png");
        commands.spawn(ProjectileBundle {
            projectile: Projectile,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
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
