// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world.start(|_|     print_number(0)              );
    world.start(|world| spawn_enemy(world, "bad guy"));

    world.update(|_|    print!("."));

    world.run();
}

fn print_number(number: usize) {
    println!("0 = {number}");
}

fn spawn_enemy(world: &mut World, name: &str) {
    world.spawn_entity();

    println!("enemy \"{name}\" appeared");
}

