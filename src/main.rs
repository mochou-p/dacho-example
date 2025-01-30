// dacho-example/src/main.rs

#![expect(clippy::multiple_crate_versions, reason = "inside dacho")]

use dacho::prelude::*;


#[derive(Default)]
struct World {
    cursor_position: PhysicalPosition<f64>,
    window_size:     Vec3,

    commands: Commands,
    meshes:   Meshes
}

impl DachoWorld for World {
    #[inline]
    #[must_use]
    fn get_mut_commands(&mut self) -> &mut Commands {
        &mut self.commands
    }

    #[inline]
    #[must_use]
    fn get_mut_meshes(&mut self) -> &mut Meshes {
        &mut self.meshes
    }
}

fn main() {
    let game = Game::<World> {
        title:            "dacho example",
        start_systems:    &[start   ],
        keyboard_systems: &[keyboard],
        mouse_systems:    &[mouse   ],
        cursor_systems:   &[cursor  ],
        ..Default::default()
    };

    game.run();
}

fn start(world: &mut World) {
    world.window_size = Vec3 { x: 800.0, y: 600.0, z: 1.0 };

    world.meshes.push(Mesh::quad(Vec3::ZERO, Vec2::ONE * 0.1));
}

fn keyboard(world: &mut World, event: &KeyEvent) {
    if event.repeat || !event.state.is_pressed() {
        return;
    }

    if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
        world.commands.submit(Command::Exit);
    }
}

fn mouse(world: &mut World, button: MouseButton, state: ElementState) {
    if !state.is_pressed() {
        return;
    }

    if matches!(button, MouseButton::Left) {
        world.meshes.push(
            Mesh::quad(
                helpers::cursor_position_to_vec3(&world.cursor_position)
                    / world.window_size
                    - consts::VEC3_HALF,
                Vec2::splat(0.2)
            )
        );
    }
}

#[inline]
fn cursor(world: &mut World, position: &PhysicalPosition<f64>) {
    world.cursor_position = *position;
}

mod consts {
    use dacho::prelude::*;

    pub static VEC3_HALF: Vec3 = Vec3::splat(0.5);
}

mod helpers {
    use dacho::prelude::*;

    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "no need for double precision")]
    pub const fn cursor_position_to_vec3(position: &PhysicalPosition<f64>) -> Vec3 {
        Vec3 { x: position.x as f32, y: position.y as f32, z: 0.0 }
    }
}

