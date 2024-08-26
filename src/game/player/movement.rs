// dacho-example/src/game/player/movement.rs

use dacho::prelude::*;

use crate::game::components::Velocity;

pub fn handle_keyboard_input(world: &mut World, key: Key, is_pressed: bool, player_id: Id) {
    match key {
        Key::KeyW | Key::KeyA | Key::KeyS | Key::KeyD => {
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

pub fn move_player_mesh_by_velocity(world: &mut World, player_id: Id) {
    let velocity_option = world.get_entity_component::<Velocity, _>(player_id, Clone::clone);

    if let Some(velocity) = velocity_option {
        if velocity.direction.is_zero() {
            return;
        }

        let mesh_id_option = world.get_entity_mut_component::<Mesh, _>(
            player_id,
            |mesh| {
                mesh.move_by(velocity.direction.normalize() * velocity.speed);

                mesh.id
            }
        );

        if let Some(mesh_id) = mesh_id_option {
            world.update_mesh(mesh_id);
        }
    }
}
