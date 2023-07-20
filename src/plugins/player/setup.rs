use bevy::prelude::*;
use crate::components::player::PlayerComponent;
use crate::plugins::player::PlayerEntity;

pub fn setup_player(
    mut commands: Commands,
) {
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::LIME_GREEN,
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            transform: Transform::from_xyz(-25., 0., 0.),
            ..default()
        })
        .insert(PlayerComponent {
            speed: 300.,
        }).id();

    commands.insert_resource(PlayerEntity { entity });
}
