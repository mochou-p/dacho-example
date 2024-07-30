// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    let mut game = Game::new("dacho example");

    let player_id    = spawn_player(&mut game.world);
    let player_id_   = player_id;
    let player_id__  = player_id;
    let player_id___ = player_id;

    game.state(
        GameState::default() as State,
        |_, old, new| state_changed(old, new)
    );

    game.start(
        move |world| {
            change_state(world, GameState::Menu);
            spawn_meshes(world, player_id);
            capitalise_items(world, player_id);
            print_items(world, player_id);
            spawn_enemy(world, "Bad guy");
        }
    );

    game.update(move |world| movement(world, player_id__));

    game.keyboard(move |world, key, action| keyboard(world, key, action.is_pressed(), player_id_));

    game.mouse_button(
        |_, button, action| {
            if button == MouseButton::Left && action.is_pressed() {
                println!("fire!");
            }
        }
    );

    game.mouse_wheel(
        move |world, _, y| {
            if y == 0.0 {
                return;
            }

            let mesh_id_option = world.get_entity_mut_component::<Mesh, _>(
                player_id___,
                |mesh| {
                    mesh.rotate_by(V3::Z * y * -0.1);

                    mesh.id
                }
            );

            if let Some(mesh_id) = mesh_id_option {
                world.update_mesh(mesh_id);
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

#[derive(Clone)]
struct Velocity {
    direction: V2,
    speed:     f32
}

impl Component for Velocity {}

impl Velocity {
    const fn new(speed: f32) -> Self {
        Self { direction: V2::ZERO, speed }
    }
}

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

fn spawn_meshes(world: &mut World, player_id: Id) {
    world.spawn_component(player_id, Velocity::new(0.01));

    world.spawn_component(
        player_id,
        Mesh::quad(
            V3::new(-1.0, 1.0, 0.0),
            V2::ONE
        )
    );

    world.spawn_component(
        player_id,
        Mesh::quad(
            V3::new(1.0, 0.0, 0.0),
            V2::ONE
        )
    );

    for x in -4..-1 {
        #[allow(clippy::cast_precision_loss)]
        world.spawn_component(
            player_id,
            Mesh::circle(
                V3::new(x as f32 * 0.8, 0.0, 0.0),
                0.5
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
    world.get_entity_mut_components::<Item, _>(
        entity_id,
        |item| {
            item.name = item.name.to_uppercase();
        }
    );
}

fn print_items(world: &World, entity_id: Id) {
    world.get_entity_components::<Item, _>(
        entity_id,
        |item| {
            println!("item \"{}\"", item.name);
        }
    );
}

fn spawn_enemy(world: &mut World, name: &str) {
    world.spawn_entity();

    println!("enemy \"{name}\" appeared");
}

fn keyboard(world: &mut World, key: Key, is_pressed: bool, player_id: Id) {
    match key {
        Key::Space => {
            if is_pressed {
                println!("jump!");
            }
        },
        Key::ArrowUp | Key::ArrowLeft | Key::ArrowDown | Key::ArrowRight | Key::KeyW | Key::KeyA | Key::KeyS | Key::KeyD => {
            world.get_entity_mut_component::<Velocity, _>(
                player_id,
                #[allow(clippy::cast_precision_loss)]
                |velocity| {
                    let sign = i32::from(is_pressed) * 2 - 1;

                    velocity.direction.x += (match key {
                        Key::ArrowLeft  | Key::KeyA => -1,
                        Key::ArrowRight | Key::KeyD =>  1,
                        _ => 0
                    } * sign) as f32;

                    velocity.direction.y += (match key {
                        Key::ArrowDown  | Key::KeyS => -1,
                        Key::ArrowUp    | Key::KeyW =>  1,
                        _ => 0
                    } * sign) as f32;
                }
            );
        },
        _ => ()
    }
}

fn movement(world: &mut World, player_id: Id) {
    let velocity_option = world.get_entity_component::<Velocity, _>(player_id, Clone::clone);

    if let Some(velocity) = velocity_option {
        if velocity.direction.is_zero() {
            return;
        }

        let mesh_id_option = world.get_entity_mut_component::<Mesh, _>(
            player_id,
            |mesh| {
                mesh.move_by(velocity.direction.extend() * velocity.speed);

                mesh.id
            }
        );

        if let Some(mesh_id) = mesh_id_option {
            world.update_mesh(mesh_id);
        }
    }
}

