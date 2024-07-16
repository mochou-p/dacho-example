// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world.start(|world| {
        let player_id = add_player(world);
        print_info(world, player_id);

        spawn_enemy(world, "bad guy");
    });

    world.update(|_| dots());

    world.run();
}

struct Info {
    name: String
}

impl Component for Info {}

fn add_player(world: &mut World) -> Id {
    let player_id = world.spawn_entity();

    world.spawn_component(player_id, Info { name: String::from("P1") });

    player_id
}

fn print_info(world: &World, entity_id: Id) {
    if let Some(component) = world.get_component::<Info>(entity_id) {
        println!("{}", component.name);
    }
}

fn spawn_enemy(world: &mut World, name: &str) {
    world.spawn_entity();

    println!("enemy \"{name}\" appeared");
}

fn dots() {
    use std::io::{stdout, Write};

    static mut I: usize = 0;

    match unsafe { I } {
        10_000_000 => {
            unsafe { I = 0; }

            print!(".");

            stdout().flush().expect("failed to flush stdout");
        },
        _ => {
            unsafe { I += 1; }
        }
    }
}

