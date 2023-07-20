use crate::components::player::PlayerComponent;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

pub fn player_move(
    mut player_query: Query<(&PlayerComponent, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player_component, mut player_transform) = player_query.single_mut();

    let mut delta = Vec3::ZERO;

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::W => {
                delta.y += player_component.speed * time.delta_seconds();
            }
            KeyCode::A => {
                delta.x -= player_component.speed * time.delta_seconds();
            }
            KeyCode::S => {
                delta.y -= player_component.speed * time.delta_seconds();
            }
            KeyCode::D => {
                delta.x += player_component.speed * time.delta_seconds();
            }
            _ => {
                continue;
            }
        }
    }

    player_transform.translation += delta;
}
