// dacho-example/src/main.rs

#[allow(clippy::wildcard_imports)]
use dacho::*;


fn main() {
    App::new("dacho example")
        .add_system(Schedule::Start,    spawn_entities        )
        .add_system(Schedule::Update,   move_players          )
        .add_system(Schedule::Keyboard, print_player_positions)
        .run();
}

#[system]
fn spawn_entities(q_world: Query<WorldComponent>) {
    q_world.one().0.borrow().get(|mut world| {
        world.spawn((MeshComponent::circle(V3::ZERO, 0.3),));

        world.spawn((Player("Miku"), Position { x: -1.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }));
        world.spawn((Player("Len"),  Position { x:  0.0, y: 0.0 }, Velocity { x: 2.0, y: 0.0 }));
        world.spawn((Player("Rin"),  Position { x:  1.0, y: 0.0 }, Velocity { x: 3.0, y: 1.0 }));
    });
}

#[system]
fn move_players(
    q_time:    Query<TimeComponent>,
    q_players: Query<(Player, Position, Velocity)>
) {
    let (time_c,) = q_time.one();
    let  time     = time_c.borrow();

    for (_, position, velocity) in q_players.all() {
        let mut pos = position.borrow_mut();
        let     vel = velocity.borrow();

        pos.x += vel.x * time.delta;
        pos.y += vel.y * time.delta;
    }
}

#[system]
fn print_player_positions(
    q_key:     Query<KeyComponent>,
    q_players: Query<(Player, Position)>
) {
    let (key_c,) = q_key.one();
    let  key     = key_c.borrow();

    if key.down && key.code == KeyCode::Space {
        for (player, position) in q_players.all() {
            let name = player.borrow().0;
            let pos  = position.borrow();

            println!("{}:\t{},\t{}", name, pos.x, pos.y);
        }
    }
}

struct Player(&'static str);

struct Position {
    x: f32,
    y: f32
}

struct Velocity {
    x: f32,
    y: f32
}

