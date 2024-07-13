// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world.spawn("dog");
    world.spawn("cat");

    let (_, player) = world.spawn("player");

    player.add(ComponentA { name: "A".to_string() });
    player.add(ComponentB);

    let (car_id, _) = world.spawn("car");
    println!("car = {}\n", world.get(car_id).expect("car is None").name);

    world.debug();
}

struct ComponentA {
    pub name: String
}

impl Component for ComponentA {
    fn name(&self) -> &str {
        &self.name
    }
}

struct ComponentB;

impl Component for ComponentB {
    fn name(&self) -> &str {
        "B"
    }
}

