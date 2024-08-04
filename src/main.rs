// dacho-example/src/main.rs

mod game;

use dacho::prelude::*;

fn main() {
    let mut app = App::new("dacho example");

    let player_id = app.world.spawn_entity();

    app.    start(game::   start(player_id));
    app.   update(game::  update(player_id));
    app. keyboard(game::keyboard(player_id));

    app.run();
}

