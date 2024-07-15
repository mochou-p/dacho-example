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
    if let Some(entity1_child1_id) = world.spawn_child_entity(entity1_id) {
        world.spawn_child_entity(entity1_child1_id);
        world.spawn_component(entity1_child1_id, ComponentB);

        world.remove_entity(entity1_child1_id); // also removes the child and component above
    }



    // callback example
    let player_id = world.spawn_entity();

    world.spawn_component(player_id, Weapon { damage: 50 });

    world.call_mut (change_weapon_damage, &[player_id], &100_isize);
    world.call     (print_player,         &[player_id]            );



    // world.debug();
}

struct ComponentA;
impl Component for ComponentA {}

struct ComponentB;
impl Component for ComponentB {}

struct Weapon {
    pub damage: isize
}
impl Component for Weapon {}

#[allow(clippy::needless_pass_by_value)]
fn change_weapon_damage(world: &mut World, ids: &[Id], data: &dyn Any) {
    let player_id = ids[0];

    world.get_mut_component::<Weapon, _>(player_id, |weapon_option| {
        if let Some(weapon) = weapon_option {
            if let Some(damage) = data.downcast_ref::<isize>() {
                weapon.damage = *damage;
            }
        }
    });
}

fn print_player(world: &World, ids: &[Id]) {
    let player_id = ids[0];

    let weapon = world.get_component::<Weapon>(player_id);

    println!("player's weapon has {} damage", weapon.expect("None").damage);
}

