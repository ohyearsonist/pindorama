use bevy::prelude::*;

pub struct PlayerPlugin;

mod setup;

#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Entity,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Startup, setup::setup_player);
    }
}
