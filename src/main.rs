// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world.start(|world| {
        let player_id = add_player(world);
        capitalise_items(world, player_id);
        print_items(world, player_id);

        spawn_enemy(world, "bad guy");
    });

    world.update(|_| dots());

    world.run();
}

struct Item {
    name: String
}

impl Component for Item {}

fn add_player(world: &mut World) -> Id {
    let player_id = world.spawn_entity();

    world.spawn_component(player_id, Item { name: String::from("weapon") });
    world.spawn_component(player_id, Item { name: String::from("potion") });

    player_id
}

fn capitalise_items(world: &mut World, entity_id: Id) {
    world.get_mut_components::<Item>(entity_id, |_, item| item.name = item.name.to_uppercase());
}

fn print_items(world: &World, entity_id: Id) {
    world.get_components::<Item>(entity_id, |_, item| println!("item \"{}\"", item.name));
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

