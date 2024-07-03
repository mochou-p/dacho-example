// dacho-example/src/main.rs

use dacho::prelude::*;

fn main() {
    World::new()
        .add(&[//                                         zoom ar        near   far
            Camera::new(V3::new(0.0, 0.0, 10.0), Camera2D(1.0, 16.0/9.0, 0.001, 10000.0))
            // Camera::default()
                .build(),
            Sphere::default()
                .build()
        ])
        .run();
}

