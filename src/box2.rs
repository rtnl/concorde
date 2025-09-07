use crate::axis::Axis;
use crate::vec2::Vec2f;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Box2f {
    pos: Vec2f,
    size: Vec2f,
}

impl Box2f {
    pub fn new(pos: Vec2f, size: Vec2f) -> Box2f {
        Self { pos, size }
    }

    pub fn pos(&self) -> Vec2f {
        self.pos
    }

    pub fn size(&self) -> Vec2f {
        self.size
    }

    pub fn split(&self, axis: Axis, ratio: f64) -> (Box2f, Box2f) {
        let f_left = Vec2f::new(ratio, 1.0).flip_if(axis == Axis::Vertical);
        let f_right = Vec2f::new(1.0 - ratio, 1.0).flip_if(axis == Axis::Vertical);

        let v_left = Self::new(self.pos, self.size * f_left);
        let v_right = Self::new(
            (self.pos + v_left.size) * Vec2f::new(1.0, 0.0).flip_if(axis == Axis::Vertical),
            self.size * f_right,
        );

        (v_left, v_right)
    }
}
