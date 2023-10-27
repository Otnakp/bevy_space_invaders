use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub sprite: SpriteBundle,
}
