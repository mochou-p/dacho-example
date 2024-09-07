// dacho-example/src/main.rs

#![allow(clippy::wildcard_imports)]

use dacho::*;

fn main() {
    let mut app = App::new("dacho example");

    app.world.spawn((Player { name: "P1", level: 1 },));
    app.world.add_system(level_up);
    app.world.run();
}

fn level_up(query: &mut (Player,)) {
    let (player,) = query;

    player.level += 1;

    println!("{} leveled up! ({})", player.name, player.level);
}

struct Player {
    name:  &'static str,
    level:          u8
}

