// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut game = Game::new("dacho example");

    game.start(
        |world| {
            let player_id = add_player(world);

            capitalise_items(world, player_id);
            print_items(world, player_id);
            spawn_enemy(world, "bad guy");
        }
    );

    game.update(|_| print!("."));

    game.run();
}

struct Item {
    name: String
}

impl Component for Item {}

fn add_player(world: &mut World) -> Id {
    let player_id = world.spawn_entity();

    world.spawn_component(player_id, Item { name: String::from("weapon") });
    world.spawn_component(player_id, Item { name: String::from("potion") });

    player_id
}

fn capitalise_items(world: &mut World, entity_id: Id) {
    world.get_entity_mut_components::<Item>(entity_id, |item| item.name = item.name.to_uppercase());
}

fn print_items(world: &World, entity_id: Id) {
    world.get_entity_components::<Item>(entity_id, |item| println!("item \"{}\"", item.name));
}

fn spawn_enemy(world: &mut World, name: &str) {
    world.spawn_entity();

    println!("enemy \"{name}\" appeared");
}

