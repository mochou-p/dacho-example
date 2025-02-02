// dacho-example/src/main.rs

#![expect(clippy::multiple_crate_versions, reason = "inside dacho")]

use dacho::prelude::*;


#[derive(Default)]
struct MyGame {
    cursor_position: PhysicalPosition<f64>,
    window_size:     Vec3,
    aspect_ratio:    Vec3,
    movement:        Vec2,
    clicked:         bool
}

fn main() {
    let game = Game::<MyGame> {
        title:            "dacho example",
        start_systems:    &[start   ],
        update_systems:   &[update  ],
        keyboard_systems: &[keyboard],
        mouse_systems:    &[mouse   ],
        cursor_systems:   &[cursor  ],
        end_systems:      &[end     ],
        ..Default::default()
    };

    game.run();
}

fn start(data: &mut Data<MyGame>) {
    data.game.window_size  = Vec3 { x: 1600.0, y: 900.0, z: 1.0 };

    data.game.aspect_ratio = Vec3 {
        x: data.game.window_size.x / data.game.window_size.y,
        y: 1.0,
        z: 1.0
    };

    data.engine.meshes.push(
        Mesh::quad(
            Vec3::ZERO,
            Vec2 { x: 1000.0 * data.game.aspect_ratio.x, y: 0.0003 }
        )
    );
    data.engine.meshes.push(
        Mesh::quad(
            Vec3::ZERO,
            Vec2 { x: 0.0003, y: 1000.0 }
        )
    );
}

fn update(data: &mut Data<MyGame>, time: &Time) {
    if data.game.movement != Vec2::ZERO {
        data.engine.camera.move_by(
            (data.game.movement.normalize() * time.delta)
                .extend(0.0)
        );

        try_draw(data);
    }
}

fn keyboard(data: &mut Data<MyGame>, event: &KeyEvent) {
    use {
        PhysicalKey::Code,
        KeyCode::{
            ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Escape,
            KeyA, KeyD, KeyQ, KeyS, KeyW, Space
        }
    };

    if event.repeat {
        return;
    }

    let s = helpers::sign_from_element_state(event.state);

    match event.physical_key {
        Code(Escape | KeyQ) => {
            data.engine.commands.submit(Command::Exit);
        },

        Code(KeyW | ArrowUp   ) => { data.game.movement.y -= 1.0 * s; },
        Code(KeyA | ArrowLeft ) => { data.game.movement.x -= 1.0 * s; },
        Code(KeyS | ArrowDown ) => { data.game.movement.y += 1.0 * s; },
        Code(KeyD | ArrowRight) => { data.game.movement.x += 1.0 * s; },

        Code(Space) => if event.state.is_pressed() {
            data.engine.camera.move_to(Vec2::ZERO);
        },
        _ => ()
    }
}

fn mouse(data: &mut Data<MyGame>, button: MouseButton, state: ElementState) {
    if matches!(button, MouseButton::Left) {
        data.game.clicked = state.is_pressed();
    }
}

fn cursor(data: &mut Data<MyGame>, position: &PhysicalPosition<f64>) {
    data.game.cursor_position = *position;

    try_draw(data);
}

fn end(_data: &mut Data<MyGame>) {
    println!("bye");
}

fn try_draw(data: &mut Data<MyGame>) {
    if data.game.clicked {
        let mut pos  = helpers::cursor_position_to_vec3(&data.game.cursor_position);
        pos         /= data.game.window_size;
        pos         -= consts::VEC3_HALF_XY;
        pos         *= data.game.aspect_ratio;
        pos         += data.engine.camera.position.truncate().extend(0.0);

        data.engine.meshes.push(
            Mesh::quad(
                pos,
                Vec2::splat(0.0075)
            )
        );
    }
}

mod consts {
    use dacho::prelude::*;

    pub static VEC3_HALF_XY: Vec3 = Vec3 { x: 0.5, y: 0.5, z: 0.0 };
}

mod helpers {
    use dacho::prelude::*;

    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "no need for double precision")]
    pub const fn cursor_position_to_vec3(position: &PhysicalPosition<f64>) -> Vec3 {
        Vec3 { x: position.x as f32, y: position.y as f32, z: 0.0 }
    }

    #[inline]
    #[must_use]
    pub fn sign_from_element_state(state: ElementState) -> f32 {
        f32::from(i8::from(state.is_pressed()) * 2 - 1)
    }
}

