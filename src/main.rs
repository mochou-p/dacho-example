// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();



    // component example
    let entity1_id = world.spawn_entity();

    world.spawn_component(entity1_id, ComponentA);
    world.spawn_component(entity1_id, ComponentB);
    world.spawn_component(entity1_id, ComponentB);
    world.spawn_component(entity1_id, ComponentB);
    world.spawn_component(entity1_id, ComponentA);

    world.remove_component ::<ComponentA>(entity1_id); // removes the first found ComponentA
    world.remove_components::<ComponentB>(entity1_id); // removes all ComponentBs



    // parent child relationship example
    let entity1_child1_id = world.spawn_child_entity(entity1_id);

    world.spawn_child_entity(entity1_child1_id);
    world.spawn_component(entity1_child1_id, ComponentB);

    world.remove_entity(entity1_child1_id); // also removes the child and component above



    // callback example
    let player_id = world.spawn_entity();

    world.call(print_player, &[player_id]); // call = temporary immediate schedule for showcase



    // world.debug();
}

struct ComponentA;

impl Component for ComponentA {
    fn print(&self) {
        println!("A");
    }
}

struct ComponentB;

impl Component for ComponentB {
    fn print(&self) {
        println!("B");
    }
}

fn print_player(world: &World, ids: &[u64]) {
    let player_id = ids[0];

    let player = world.get_entity(player_id);

    dbg!(&player);
}

