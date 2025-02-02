// dacho-example/src/main.rs

#![expect(clippy::multiple_crate_versions, reason = "inside dacho")]

use dacho::prelude::*;


#[derive(Default)]
struct World {
    cursor_position: PhysicalPosition<f64>,
    window_size:     Vec3,
    aspect_ratio:    Vec3,
    movement:        Vec2,
    clicked:         bool,

    commands: Commands,
    meshes:   Meshes,
    camera:   Camera
}

impl DachoWorld for World {
    #[inline]
    fn get_mut_commands(&mut self) -> &mut Commands {
        &mut self.commands
    }

    #[inline]
    fn get_mut_meshes(&mut self) -> &mut Meshes {
        &mut self.meshes
    }

    #[inline]
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

fn main() {
    let game = Game::<World> {
        title:            "dacho example",
        start_systems:    &[start   ],
        update_systems:   &[update  ],
        keyboard_systems: &[keyboard],
        mouse_systems:    &[mouse   ],
        cursor_systems:   &[cursor  ],
        ..Default::default()
    };

    game.run();
}

fn start(world: &mut World) {
    world.window_size  = Vec3 { x: 1600.0, y: 900.0, z: 1.0 };

    world.aspect_ratio = Vec3 {
        x: world.window_size.x / world.window_size.y,
        y: 1.0,
        z: 1.0
    };

    world.meshes.push(
        Mesh::quad(
            Vec3::ZERO,
            Vec2 { x: 1000.0 * world.aspect_ratio.x, y: 0.0003 }
        )
    );
    world.meshes.push(
        Mesh::quad(
            Vec3::ZERO,
            Vec2 { x: 0.0003, y: 1000.0 }
        )
    );
}

fn update(world: &mut World, time: &Time) {
    if world.movement != Vec2::ZERO {
        world.camera.move_by(
            (world.movement.normalize() * time.delta)
                .extend(0.0)
        );

        try_draw(world);
    }
}

fn keyboard(world: &mut World, event: &KeyEvent) {
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
            world.commands.submit(Command::Exit);
        },

        Code(KeyW | ArrowUp   ) => { world.movement.y -= 1.0 * s; },
        Code(KeyA | ArrowLeft ) => { world.movement.x -= 1.0 * s; },
        Code(KeyS | ArrowDown ) => { world.movement.y += 1.0 * s; },
        Code(KeyD | ArrowRight) => { world.movement.x += 1.0 * s; },

        Code(Space) => if event.state.is_pressed() {
            world.camera.move_to(Vec2::ZERO);
        },
        _ => ()
    }
}

fn mouse(world: &mut World, button: MouseButton, state: ElementState) {
    if matches!(button, MouseButton::Left) {
        world.clicked = state.is_pressed();
    }
}

fn cursor(world: &mut World, position: &PhysicalPosition<f64>) {
    world.cursor_position = *position;

    try_draw(world);
}

fn try_draw(world: &mut World) {
    if world.clicked {
        let mut pos  = helpers::cursor_position_to_vec3(&world.cursor_position);
        pos         /= world.window_size;
        pos         -= consts::VEC3_HALF_XY;
        pos         *= world.aspect_ratio;
        pos         += world.camera.position.truncate().extend(0.0);

        world.meshes.push(
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

