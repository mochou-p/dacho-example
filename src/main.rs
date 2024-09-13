// dacho-example/src/main.rs

#[allow(clippy::wildcard_imports)]
use dacho::*;

fn main() {
    let mut app = App::new("dacho example");

    app.add_system(start);
    app.add_system(print_player_weapons);
    app.add_system(damage_boss);

    app.run();
}

fn start((query,): (Query<(WorldComponent,)>,)) {
    let world_comp = query.one().0.borrow().get();
    let mut world  = world_comp.borrow_mut();

    world.spawn((
        Player       { name: "Yonezu" },
        ActiveWeapon ( Weapon { name: "Stick", damage: 3.14, rarity: Rarity::Trash } ),
        Weapon       { name: "Ashbringer", damage: 917.37, rarity: Rarity::Legendary }
    ));

    world.spawn((
        Player { name: "Suda" },
    ));

    world.spawn((
        Boss { health: 100.0 },
    ));
}

fn print_player_weapons((player_query,): (Query<(Player,)>,)) {
    for entity in player_query.entities() {
        let Some(player) = entity.get_component::<Player>() else { continue; };

        let  weapons_o = entity.get_components::<Weapon>();
        let active_w_o = entity.get_component ::<ActiveWeapon>();

        print!("{} ", player.borrow().name);

        let mut amount  = 0;
        let mut message = String::new();

        if let Some(active_w) = active_w_o {
            let w    = active_w.borrow();

            amount  += 1;
            message += &format!("\x1b[1m*{} ({:?})*\x1b[0m, ", w.0.name, w.0.rarity);
        }

        if let Some(weapons) = weapons_o {
            amount += weapons.len();

            for weapon in weapons {
                let w = weapon.borrow();

                message += &format!("{} ({:?}), ", w.name, w.rarity);
            }
        }

        match amount {
            0 => println!("has no weapons"),
            _ => println!("has {amount} weapons: {{ {message}}}")
        }
    }
}

fn damage_boss((q1, q2): (Query<(Boss,)>, Query<(Player, ActiveWeapon)>)) {
    let q1_components = q1.one().0;
    let mut boss      = q1_components.borrow_mut();

    print!("Boss health: {} -> ", boss.health);

    for (_, weapon) in q2.all() {
        boss.health -= weapon.borrow().0.damage;
    }

    println!("{}", boss.health);
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

