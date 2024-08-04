// dacho-example/src/game/components/velocity.rs

use dacho::prelude::*;

#[derive(Clone)]
pub struct Velocity {
    pub direction: V3,
    pub speed:     f32
}

impl Component for Velocity {}

impl Velocity {
    pub const fn new(speed: f32) -> Self {
        Self { direction: V3::ZERO, speed }
    }
}

