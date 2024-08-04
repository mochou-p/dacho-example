// dacho-example/src/game/mod.rs

mod components;
mod player;

use dacho::prelude::*;

pub fn start(player_id: Id) -> impl FnOnce(&mut World) {
    move |world| {
        player::spawn_player_components(
            world,
            player_id
        );
    }
}

pub fn update(player_id: Id) -> impl Fn(&mut World) {
    move |world| {
        player::move_player_mesh_by_velocity(
            world,
            player_id
        );
    }
}

pub fn keyboard(player_id: Id) -> impl Fn(&mut World, Key, bool) {
    move |world, key, is_pressed| {
        player::handle_keyboard_input(
            world,
            key,
            is_pressed,
            player_id
        );
    }
}

