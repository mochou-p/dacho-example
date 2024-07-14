// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();



    // entity and component example
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

    world.spawn_component(player_id, Weapon { damage: 50 });

    world.call(print_weapon_damage, &[player_id]); // passtrough IDs
    world.call(print_player,        &[player_id]);



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

struct Weapon {
    pub damage: isize
}

impl Component for Weapon {
    fn print(&self) {
        println!("weapon damage is {}", self.damage);
    }
}

fn print_weapon_damage(world: &World, ids: &[u64]) {
    let player_id = ids[0];

    let component = world.get_component::<Weapon>(player_id);

    component.print();
}

fn print_player(world: &World, ids: &[u64]) {
    let player_id = ids[0];

    let player = world.get_entity(player_id);

    dbg!(&player);
}

