// dacho-example/src/game/player/spawn.rs

use dacho::prelude::*;

use crate::game::components::Velocity;

#[allow(clippy::module_name_repetitions)]
pub fn spawn_player_components(world: &mut World, player_id: Id) {
    world.spawn_component(player_id, Mesh::quad(V3::ZERO, V2::ONE));
    world.spawn_component(player_id, Velocity::new(0.01));
}

