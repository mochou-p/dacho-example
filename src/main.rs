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



    // callback examples
    let player_id = world.spawn_entity();

    world.spawn_component(player_id, Weapon { damage: 50                     });
    world.spawn_component(player_id, Item   { name:   String::from("potion") });
    world.spawn_component(player_id, Item   { name:   String::from("key")    });

    world.spawn_components(player_id, 2, Chest);
    world.spawn_components(player_id, 3, Chest);
    world.spawn_components(player_id, 5, Chest);

    // call = callbacks run instantly
    // callback data type is fully dependant on your function parameter
    world.call_mut(change_damage,   (player_id, 100));
    world.call_mut(uppercase_items,  player_id      );

    world.call(print_damage, player_id);
    world.call(print_items,  player_id);
    world.call(print_chests, player_id);

    // start = runs when the world is ran
    // callback data type is Any
    // so make sure to strongly type your data
    // to not fail the downcast you will need
    // inside the callback
    world.start(print_number, 0_usize);
    //                          ^^^^^
    //                           vvvvvv
    world.start_mut(spawn_enemy, String::from("bad guy"));


    world.run();
}

fn print_number(_: &World, data: &Box<dyn Any>) {
    if let Some(number) = data.downcast_ref::<usize>() {
        println!("0 = {number}");
    }
}

fn spawn_enemy(world: &mut World, data: &Box<dyn Any>) {
    if let Some(name) = data.downcast_ref::<String>() {
        world.spawn_entity();

        println!("enemy \"{name}\" appeared");
    }
}

fn change_damage(world: &mut World, (player_id, new_damage): (Id, isize)) {
    world.get_mut_component::<Weapon, _>(player_id, |weapon| {
        weapon.damage = new_damage;
    });
}

fn uppercase_items(world: &mut World, player_id: Id) {
    world.get_mut_components::<Item, _>(player_id, |item| {
        item.name = item.name.to_uppercase();
    });
}

fn print_damage(world: &World, player_id: Id) {
    if let Some(weapon) = world.get_component::<Weapon>(player_id) {
        println!("player's damage = {}", weapon.damage);
    }
}

fn print_items(world: &World, player_id: Id) {
    print!("player's items = {{ ");

    for item in &world.get_components::<Item>(player_id) {
        print!("{}, ", item.name);
    }

    println!("}}");
}

fn print_chests(world: &World, player_id: Id) {
    println!("player has {} chests", world.get_components::<Chest>(player_id).len());
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

#[derive(Clone, Copy)]
struct Chest;
impl Component for Chest {}

