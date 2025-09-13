use crate::axis::Axis;
use crate::vec2::Vec2f;
use num_traits::{Inv, Zero};

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Box2f {
    pos: Vec2f,
    size: Vec2f,
}

impl Box2f {
    pub fn new(pos: Vec2f, size: Vec2f) -> Self {
        Self { pos, size }
    }

    pub fn new_root() -> Self {
        Self::new(Vec2f::zero(), Vec2f::of(1.0))
    }

    pub fn pos(&self) -> Vec2f {
        self.pos
    }

    pub fn size(&self) -> Vec2f {
        self.size
    }

    pub fn split(&self, axis: Axis, n: i32) -> Vec<Box2f> {
        let mut result = vec![];

        let vert = axis == Axis::Vertical;
        let n_f = n as f64;
        let n_i = if n_f.is_zero() { 1.0 } else { n_f.inv() };

        let mut start = Vec2f::zero();
        let chunk = Vec2f::new(n_i, 1.0).flip_if(vert);
        let diff = Vec2f::new(1.0, 0.0).flip_if(vert) * chunk;

        for i in 0..n {
            let v_pos = start;
            let v_size = chunk;

            start = start + diff;

            result.push(Box2f::new(v_pos, v_size));
        }

        result
    }
}
