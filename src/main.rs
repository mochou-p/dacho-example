// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world.add("dog");
    world.add("cat");

    let player_id = world.add("player");
    let player    = world.get(player_id).expect("player == None");
    println!("this should be player -> {}\n", player.name);

    world.add("car");

    world.debug();
}

