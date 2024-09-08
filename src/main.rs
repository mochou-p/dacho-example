// dacho-example/src/main.rs

#![allow(clippy::wildcard_imports)]

use dacho::*;

fn main() {
    let mut app = App::new("dacho example");

    app.world.spawn((
        Player       { name: "Yonezu" },
        ActiveWeapon ( Weapon { name: "Stick", damage: 3.14, rarity: Rarity::Trash } ),
        Weapon       { name: "Ashbringer", damage: 917.37, rarity: Rarity::Legendary }
    ));

    app.world.spawn((
        Player { name: "Suda" },
    ));

    app.world.spawn((
        Boss { health: 100.0 },
    ));

    app.world.add_system(print_player_weapons);
    app.world.add_system(damage_boss);

    app.world.run();
}

fn print_player_weapons((player_query,): (Query<(Player,)>,)) {
    if let Some(entities) = player_query.all() {
        for entity in entities {
            let Some(player) = entity.get_component::<Player>() else { continue; };

            if let Some(weapons) = entity.get_components::<Weapon>() {
                print!("Player `{}` has {} weapons {{ ", player.name, weapons.len());

                if let Some(active_weapon) = entity.get_component::<ActiveWeapon>() {
                    print!("\x1b[1m*{} ({:?})*\x1b[0m, ", active_weapon.0.name, active_weapon.0.rarity);
                }

                for weapon in &weapons {
                    print!("{} ({:?}), ", weapon.name, weapon.rarity);
                }

                println!("}}");
            } else {
                println!("Player `{}` has no weapons", player.name);
            }
        }
    }
}

fn damage_boss((boss_query, player_query): (Query<(Boss,)>, Query<(Player, ActiveWeapon)>)) {
    let mut damage = 0.0;

    if let Some(entities) = player_query.all() {
        for entity in entities {
            let Some(weapon) = entity.get_component::<ActiveWeapon>() else { continue; };

            damage += weapon.0.damage;
        }
    }

    if damage > 0.0 {
        let Some(entity) = boss_query.single()            else { return; };
        let Some(boss)   = entity.get_component::<Boss>() else { return; };

        println!("Boss health: {} -> {}", boss.health, boss.health - damage);
    }
}

struct Player {
    name: &'static str
}

struct Weapon {
    name:   &'static str,
    damage: f32,
    rarity: Rarity
}

struct ActiveWeapon(Weapon);

struct Boss {
    health: f32
}

#[derive(Debug)]
enum Rarity { Trash, _Common, _Uncommon, _Rare, _Epic, Legendary }

