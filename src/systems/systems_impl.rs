use crate::systems::constants::*;
use crate::systems::player::Player;
use bevy::prelude::*;
use bevy::window::WindowResized;
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

pub fn on_resize_system(
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

pub fn check_borders(
    mut query: Query<(&mut Transform, &Sprite), With<Player>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let width = window.resolution.width();
    let (mut transform, _sprite) = query.single_mut();

    if transform.translation.x > width / 2.0 {
        transform.translation.x = width / 2.0;
    } else if transform.translation.x < -width / 2.0 {
        transform.translation.x = -width / 2.0;
    }
}
