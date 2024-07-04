// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    World::new()
        .add(&[
            Camera(
                Perspective {
                    position:     V3::new(2.5, 1.0, 7.5),
                    fov:          45.0,
                    aspect_ratio: 16.0/9.0,
                    near:         0.0001,
                    far:          10000.0
                }
            ),

            Shape2D(Quad   { position: V3::X * -2, size:   V2::ONE        }),
            Shape2D(Circle { position: V3::X *  2, radius: 0.5, points: 3 }),
            Shape2D(Circle { position: V3::X *  3, radius: 0.5, points: 4 }),
            Shape2D(Circle { position: V3::X *  4, radius: 0.5, points: 5 }),
            Shape2D(Circle { position: V3::X *  5, radius: 0.5, points: 6 }),
            Shape2D(Circle { position: V3::X *  6, radius: 0.5, points: 7 }),
            Shape2D(Circle { position: V3::X *  7, radius: 0.5, points: 8 }),

            Shape3D(Cube   { position: V3::ZERO,     size:   V3::new(2.0, 0.1, 2.0)       }),
            Shape3D(Sphere { position: V3::Y * 0.55, radius: 0.5, sectors: 32, stacks: 18 })
        ])
        .run();
}
