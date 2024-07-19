// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut game = Game::new("dacho example");

    game.state(GameState::default() as State, |_, old, new| state_changed(old, new));

    game.start(
        |world| {
            change_state(world, GameState::Menu);

            let player_id = add_player(world);

            capitalise_items(world, player_id);
            print_items(world, player_id);
            spawn_enemy(world, "Bad guy");
        }
    );

    game.update(|_| print!("."));

    game.run();
}

#[derive(Default, Debug)]
enum GameState {
    #[default]
    Loading,
    Menu,
    InGame,
    Paused
}

impl TryFrom<State> for GameState {
    type Error = String;

    fn try_from(value: State) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Loading),
            1 => Ok(Self::Menu),
            2 => Ok(Self::InGame),
            3 => Ok(Self::Paused),
            _ => Err(format!("cannot convert {value} to GameState"))
        }
    }
}

struct Item {
    name: String
}

impl Component for Item {}

fn state_changed(old_state: State, new_state: State) {
    let old_game_state = GameState::try_from(old_state).expect("old_game_state");
    let new_game_state = GameState::try_from(new_state).expect("new_game_state");

    #[allow(clippy::single_match)]
    match (old_game_state, new_game_state) {
        (GameState::Loading, GameState::Menu) => println!("game started!"),
        _ => ()
    }
}

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

fn change_state(world: &mut World, new_state: GameState) {
    world.set_state(new_state as State);
}

