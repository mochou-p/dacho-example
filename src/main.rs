// dacho-example/src/main.rs

#![allow(clippy::wildcard_imports)]

use dacho::*;

fn main() {
    let mut app = App::new("dacho example");

    app.world.spawn((
        Player { name: "Yonezu" },
        Weapon { name: "Stick",      rarity: Rarity::Trash     },
        Weapon { name: "Ashbringer", rarity: Rarity::Legendary }
    ));

    app.world.spawn((
        Player { name: "Suda" },
    ));

    app.world.add_system(print_player_weapons);
    app.world.run();
}

fn print_player_weapons(query: Query<(Player,)>) {
    for entity in query.all() {
        let Some(player) = entity.get_component::<Player>() else { continue; };

        if let Some(weapons) = entity.get_components::<Weapon>() {
            print!("Player `{}` has {} weapons {{ ", player.name, weapons.len());

            for weapon in &weapons {
                print!("{} ({:?}), ", weapon.name, weapon.rarity);
            }

            println!("}}");
        } else {
            println!("Player `{}` has no weapons", player.name);
        }
    }
}

struct Player {
    name: &'static str
}

struct Weapon {
    name:   &'static str,
    rarity: Rarity
}

#[derive(Debug)]
enum Rarity { Trash, _Common, _Uncommon, _Rare, _Epic, Legendary }

