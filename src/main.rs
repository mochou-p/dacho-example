// dacho-example/src/main.rs

use dacho::prelude::*;

use std::any::Any;

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

    world.call_mut (change_weapon_damage, &[player_id], &100_isize);
    world.call     (print_player,         &[player_id]            );



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

#[allow(clippy::needless_pass_by_value)]
fn change_weapon_damage(world: &mut World, ids: &[u64], data: &dyn Any) {
    let player_id = ids[0];

    world.get_mut_component::<Weapon, _>(player_id, |weapon| {
        weapon.damage = *data.downcast_ref::<isize>().expect("downcast None");
    });
}

fn print_player(world: &World, ids: &[u64]) {
    let player_id = ids[0];

    let weapon = world.get_component::<Weapon>(player_id);

    println!("player's weapon has {} damage", weapon.expect("None").damage);
}

