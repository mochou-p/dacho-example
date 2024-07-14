// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    let parent_1_id = world.spawn_entity();
    let  child_1_id = world.spawn_child_entity(parent_1_id);
    let _child_2_id = world.spawn_child_entity(parent_1_id);

    world.spawn_component(child_1_id, ComponentA);
    world.spawn_component(child_1_id, ComponentB);

    world.debug();
}

struct ComponentA;

impl Component for ComponentA {
    fn name(&self) -> &str {
        "A"
    }
}

struct ComponentB;

impl Component for ComponentB {
    fn name(&self) -> &str {
        "B"
    }
}

