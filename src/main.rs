// dacho-example/src/main.rs

#![expect(clippy::multiple_crate_versions, reason = "lol")]

use dacho::{App, Query, system};


fn main() {
    let mut app = App::new("dacho example");

    app.world.spawn((Player::new("Hatsune Miku"),));
    app.world.spawn((Player::new("Kagamine Rin"),));
    app.world.spawn((Player::new("Kagamine Len"),));

    app.insert(lower_case_players);
    app.insert(print_players);

    app.run();
}

#[system]
fn lower_case_players(q_players: Query<Player>) {
    for player in unsafe { q_players.iter_mut() } {
        player.name = player.name.to_lowercase();
    }
}

#[system]
fn print_players(q_players: Query<Player>) {
    for player in q_players.iter() {
        print!("{}, ", player.name);
    }
    println!();
}

struct Player {
    name: String
}

impl Player {
    fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}

