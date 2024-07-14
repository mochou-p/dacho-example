// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    let      e1_id = world.spawn_entity();
    let    e1c1_id = world.spawn_child_entity(e1_id);
    let _e1c1c1_id = world.spawn_child_entity(e1c1_id);

    world.remove_entity(e1c1_id);

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

