use crate::GravityField;
use bevy::math::Vec2;

pub struct Trajectory<'g> {
    pub gravity: &'g GravityField,
    pub state: TrajectoryNode,
    pub mass: f32,
    pub delta_secs: f32,
}

impl Trajectory<'_> {
    pub fn next_guaranteed(&mut self) -> TrajectoryNode {
        self.state.velocity += self.mass * self.gravity.acceleration_at(self.state.translation());

        self.state.translation += self.state.velocity() * self.delta_secs;
        self.state
    }
}

impl Iterator for Trajectory<'_> {
    type Item = TrajectoryNode;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_guaranteed())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TrajectoryNode {
    translation: Vec2,
    velocity: Vec2,
}

impl TrajectoryNode {
    pub fn from_translation_velocity(translation: Vec2, velocity: Vec2) -> Self {
        Self {
            translation,
            velocity,
        }
    }

    pub fn translation(&self) -> Vec2 {
        self.translation
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    /// output will always be positive
    pub fn speed(&self) -> f32 {
        self.velocity().length()
    }
}
