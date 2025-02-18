// dacho-example/src/main.rs

#![expect(clippy::multiple_crate_versions, reason = "inside dacho")]

use core::time::Duration;

use dacho::prelude::*;


type D = Data<GameData, GameEvent>;
type E = Event<GameEvent>;

#[derive(Default)]
struct GameData {
    camera_movement: Vec3
}

enum GameEvent {
    SayHi,
    SayHey
}

fn main() {
    Game {
        event_handler,
        fixed_update: Some(1.0), // in seconds
        ..default()
    }.run();
}

fn event_handler(data: &mut D, event: &E) {
    match event {
        Event::Engine(engine_event) => engine_event_handler(data, engine_event),
        Event::Game(game_event)     =>   game_event_handler(        game_event)
    }
}

fn engine_event_handler(data: &mut D, engine_event: &EngineEvent) {
    match engine_event {
        EngineEvent::Start => start(data),
        EngineEvent::FixedUpdate { .. } => {
            println!("{:.2}s", data.engine.time.elapsed);
        },
        EngineEvent::Update   { .. }        => update(data),
        EngineEvent::Device   { event, .. } => device(data, event),
        EngineEvent::Keyboard { key, is_pressed, repeat, .. } =>
            keyboard(data, key, *is_pressed, *repeat),
        _ => ()
    }
}

fn game_event_handler(game_event: &GameEvent) {
    println!("{}", match game_event {
        GameEvent::SayHi  => { "hi"  },
        GameEvent::SayHey => { "hey" }
    });
}

fn start(data: &mut D) {
    let meshes = &mut data.engine.meshes;

    meshes.push(Mesh::cube  (Vec3::ZERO,               1.0 ));
    meshes.push(Mesh::cube  (Vec3::X,                  0.3 ));
    meshes.push(Mesh::sphere(Vec3::Y,                  0.15));
    meshes.push(Mesh::cube  (Vec3::Z * -5.0 + Vec3::Y, 0.1 ));
    meshes.push(Mesh::cube  (Vec3::Z * -5.0 - Vec3::Y, 0.1 ));

    data.engine.commands.extend([
        Command::SetCursorVisible(false),
        Command::SetCursorGrab(CursorGrabMode::Confined)
    ]);
}

fn update(data: &mut D) {
    if data.game.camera_movement != Vec3::ZERO {
        data.engine.camera.y_angle_relative_move_by(
            data.game.camera_movement * data.engine.time.delta
        );
    }
}

fn keyboard(data: &mut D, key: &Key, is_pressed: bool, repeat: bool) {
    if repeat {
        return;
    }

    let PhysicalKey::Code(code) = key.physical else { return; };

    match code {
        KeyCode::Escape => {
            data.engine.commands.push(Command::Exit);
        },
        KeyCode::Tab => {
            if is_pressed {
                data.engine.events.do_after(
                    GameEvent::SayHi,
                    Duration::from_secs(3),
                    &data.engine.time
                );
            }
        },
        KeyCode::Enter => {
            if is_pressed {
                data.engine.events.do_after(
                    GameEvent::SayHey,
                    Duration::from_secs(1),
                    &data.engine.time
                );
            }
        },
        KeyCode::KeyW
        | KeyCode::KeyA
        | KeyCode::KeyS
        | KeyCode::KeyD
        | KeyCode::Space
        | KeyCode::ShiftLeft => {
            // false/true -> 0/1 -> -1/1
            let sign     = f32::from(i8::from(is_pressed) * 2 - 1);
            let speed    = 2.5 * sign;
            let movement = &mut data.game.camera_movement;

            match code {
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

fn device(data: &mut D, event: &DeviceEvent) {
    if let DeviceEvent::MouseMotion { delta } = event {
        mouse_motion(data, delta);
    }
}

fn mouse_motion(data: &mut D, delta: &(f64, f64)) {
    #[expect(clippy::cast_possible_truncation, reason = "dont need double precision")]
    let rotation_delta = Vec3 { x: -delta.1 as f32, y: delta.0 as f32, z: 0.0 } * 0.005;

    data.engine.camera.rotate_by(rotation_delta);
}
