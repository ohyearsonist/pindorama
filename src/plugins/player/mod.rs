use bevy::prelude::*;

pub struct PlayerPlugin;

mod movement;
mod setup;

#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Entity,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::setup_player);
        app.add_systems(Update, movement::player_move);
    }
}
