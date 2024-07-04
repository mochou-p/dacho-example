// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    World::new()
        .add(&[
            Camera(
                Perspective {
                    position:     V3::Z * 7.0,
                    fov:          45.0,
                    aspect_ratio: 16.0/9.0,
                    near:         0.0001,
                    far:          10000.0
                }
            ),
            Shape(Cube   { position: V3::ZERO,     size:   V3::new(2.0, 0.1, 2.0) }),
            Shape(Sphere { position: V3::Y * 0.55, radius: 0.5                    })
        ])
        .run();
}

