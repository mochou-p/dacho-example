// dacho-example/src/game/player/spawn.rs

use dacho::prelude::*;

use crate::game::components::Velocity;

#[allow(clippy::module_name_repetitions)]
pub fn spawn_player_components(world: &mut World, player_id: Id) {
    world.spawn_component(player_id, Mesh::circle(V3::ZERO, 0.5));
    world.spawn_component(player_id, Velocity::new(0.01));

    world.spawn_child_mesh(player_id, Mesh::quad(V3::Y * 0.55, V2::new(1.0, 0.3)));
    world.spawn_child_mesh(player_id, Mesh::quad(V3::Y * 0.65, V2::new(0.5, 0.3)));
}

