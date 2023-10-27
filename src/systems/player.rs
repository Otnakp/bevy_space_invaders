use bevy::prelude::*;
#[derive(Component)]
pub struct Player;
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite: SpriteBundle,
}
