// dacho-example/src/main.rs

#![expect(clippy::multiple_crate_versions,  reason = "inside dacho")]

use dacho::prelude::*;


#[derive(Default)]
struct MyGame {
    camera_movement: Vec3
}

fn main() {
    let game = Game::<MyGame> {
        title:            "dacho example",
        start_systems:    &[start   ],
        update_systems:   &[update  ],
        keyboard_systems: &[keyboard],
        motion_systems:   &[motion  ],
        ..Default::default()
    };

    game.run();
}

fn start(data: &mut Data<MyGame>) {
    data.engine.meshes.push(Mesh::cube  (Vec3::ZERO,               1.0 ));
    data.engine.meshes.push(Mesh::cube  (Vec3::X,                  0.3 ));
    data.engine.meshes.push(Mesh::sphere(Vec3::Y,                  0.15));
    data.engine.meshes.push(Mesh::cube  (Vec3::Z * -5.0 + Vec3::Y, 0.1 ));
    data.engine.meshes.push(Mesh::cube  (Vec3::Z * -5.0 - Vec3::Y, 0.1 ));

    data.engine.commands.extend([
        Command::SetCursorVisible(false),
        Command::SetCursorGrab(CursorGrabMode::Confined)
    ]);
}

fn update(data: &mut Data<MyGame>, time: &Time) {
    if data.game.camera_movement != Vec3::ZERO {
        data.engine.camera.y_angle_relative_move_by(
            data.game.camera_movement
                * time.delta
        );
    }
}

fn keyboard(data: &mut Data<MyGame>, event: &KeyEvent) {
    if event.repeat {
        return;
    }

    let PhysicalKey::Code(key_code) = event.physical_key else { return; };

    match key_code {
        KeyCode::Escape => {
            data.engine.commands.push(
                Command::Exit
            );
        },
        KeyCode::KeyW
        | KeyCode::KeyA
        | KeyCode::KeyS
        | KeyCode::KeyD
        | KeyCode::Space
        | KeyCode::ShiftLeft => {
            let sign = {
                let is_pressed = event.state.is_pressed(); // false/true
                let n          = i8::from(is_pressed);     //     0/1

                f32::from(n * 2 - 1)                       //  -1.0/1.0
            };

            let speed    = 2.5 * sign;
            let movement = &mut data.game.camera_movement;

            match key_code {
                KeyCode::KeyW      => { movement.z += speed; },
                KeyCode::KeyA      => { movement.x -= speed; },
                KeyCode::KeyS      => { movement.z -= speed; },
                KeyCode::KeyD      => { movement.x += speed; },
                KeyCode::Space     => { movement.y -= speed; },
                KeyCode::ShiftLeft => { movement.y += speed; },
                _ => ()
            }
        },
        _ => ()
    }
}

fn motion(data: &mut Data<MyGame>, delta: &(f64, f64)) {
    #[expect(clippy::cast_possible_truncation, reason = "dont need double precision")]
    let rotation_delta = Vec3 { x: -delta.1 as f32, y: delta.0 as f32, z: 0.0 } * 0.005;

    data.engine.camera.rotate_by(rotation_delta);
}

