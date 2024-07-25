// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut game = Game::new("dacho example");

    game.state(
        GameState::default() as State,
        |_, old, new| state_changed(old, new)
    );

    game.start(
        |world| {
            change_state(world, GameState::Menu);
            spawn_meshes(world);

            let player_id = spawn_player(world);

            capitalise_items(world, player_id);
            print_items(world, player_id);
            spawn_enemy(world, "Bad guy");
        }
    );

    game.keyboard(
        |_, code, action| {
            if code == KeyCode::Space && action.is_pressed() {
                println!("jump!");
            }
        }
    );

    game.mouse_button(
        |_, button, action| {
            if button == MouseButton::Left && action.is_pressed() {
                println!("fire!");
            }
        }
    );

    game.mouse_wheel(
        |_, _, y| {
            if y > 0.0 {
                println!("zoom in");
            } else if y < 0.0 {
                println!("zoom out");
            }
        }
    );

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
    let old_game_state = GameState::try_from(old_state)
        .expect("old_game_state");

    let new_game_state = GameState::try_from(new_state)
        .expect("new_game_state");

    #[allow(clippy::single_match)]
    match (old_game_state, new_game_state) {
        (GameState::Loading, GameState::Menu) => println!("game started!"),
        _ => ()
    }
}

fn change_state(world: &mut World, new_state: GameState) {
    world.set_state(new_state as State);
}

fn spawn_meshes(world: &mut World) {
    let meshes_id = world.spawn_entity();

    world.spawn_component(
        meshes_id,
        Mesh::quad(
            V3::new(-1.0, 1.0, 0.0),
            V2::ONE
        )
    );

    world.spawn_component(
        meshes_id,
        Mesh::quad(
            V3::new(1.0, 0.0, 0.0),
            V2::ONE
        )
    );

    for x in -4..-1 {
        #[allow(clippy::cast_precision_loss)]
        world.spawn_component(
            meshes_id,
            Mesh::circle(
                V3::new(x as f32 * 0.8, 0.0, 0.0),
                0.3
            )
        );
    }
}

fn spawn_player(world: &mut World) -> Id {
    let player_id = world.spawn_entity();

    world.spawn_component(player_id, Item { name: String::from("weapon") });
    world.spawn_component(player_id, Item { name: String::from("potion") });

    player_id
}

fn capitalise_items(world: &mut World, entity_id: Id) {
    world.get_entity_mut_components::<Item>(
        entity_id,
        |item| item.name = item.name.to_uppercase()
    );
}

fn print_items(world: &World, entity_id: Id) {
    world.get_entity_components::<Item>(
        entity_id,
        |item| println!("item \"{}\"", item.name)
    );
}

fn spawn_enemy(world: &mut World, name: &str) {
    world.spawn_entity();

    println!("enemy \"{name}\" appeared");
}

