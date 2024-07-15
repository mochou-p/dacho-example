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
    if let Some(entity1_child1_id) = world.spawn_child_entity(entity1_id) {
        world.spawn_child_entity(entity1_child1_id);
        world.spawn_component(entity1_child1_id, ComponentB);

        world.remove_entity(entity1_child1_id); // also removes the child and component above
    }



    // callback example
    let player_id = world.spawn_entity();

    world.spawn_component(player_id, Weapon { damage: 50                     });
    world.spawn_component(player_id, Item   { name:   String::from("potion") });
    world.spawn_component(player_id, Item   { name:   String::from("key")    });

    world.call_mut (change_damage,   &[player_id], 100);
    world.call_mut (uppercase_items, &[player_id], () );
    world.call     (print_damage,    &[player_id]     );
    world.call     (print_items,     &[player_id]     );



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

struct Item {
    pub name: String
}
impl Component for Item {}

fn change_damage(world: &mut World, ids: &[Id], new_damage: isize) {
    if let Some(player_id) = ids.first() {
        world.get_mut_component::<Weapon, _>(*player_id, |weapon| {
            weapon.damage = new_damage;
        });
    }
}

fn uppercase_items(world: &mut World, ids: &[Id], _: ()) {
    if let Some(player_id) = ids.first() {
        world.get_mut_components::<Item, _>(*player_id, |item| {
            item.name = item.name.to_uppercase();
        });
    }
}

fn print_damage(world: &World, ids: &[Id]) {
    if let Some(player_id) = ids.first() {
        if let Some(weapon) = world.get_component::<Weapon>(*player_id) {
            println!("player's damage = {}", weapon.damage);
        }
    }
}

fn print_items(world: &World, ids: &[Id]) {
    if let Some(player_id) = ids.first() {
        print!("player's items: ");

        for item in &world.get_components::<Item>(*player_id) {
            print!("{}, ", item.name);
        }

        println!();
    }
}

