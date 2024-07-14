// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();


    let e1_id = world.spawn_entity();

    world.spawn_component(e1_id, ComponentA);
    world.spawn_component(e1_id, ComponentB);
    world.spawn_component(e1_id, ComponentB);
    world.spawn_component(e1_id, ComponentB);
    world.spawn_component(e1_id, ComponentA);

    world.remove_component::<ComponentA>(e1_id);  // removes the first found ComponentA
    world.remove_components::<ComponentB>(e1_id); // removes all ComponentBs


    let e1c1_id = world.spawn_child_entity(e1_id);

    world.spawn_child_entity(e1c1_id);
    world.spawn_component(e1c1_id, ComponentB);

    world.remove_entity(e1c1_id); // also removes the child and component above


    world.debug();
}

#[allow(dead_code)]
struct ComponentA;

impl Component for ComponentA {
    fn name(&self) -> &str {
        "A"
    }
}

#[allow(dead_code)]
struct ComponentB;

impl Component for ComponentB {
    fn name(&self) -> &str {
        "B"
    }
}

