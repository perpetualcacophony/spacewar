use bevy::math::Vec2;
const GRAVITY: f32 = 1.0 / 50.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mass {
    pub translation: Vec2,
    pub mass: f32,
}

impl Mass {
    fn acceleration_to(&self, from_point: Vec2) -> Vec2 {
        let displacement = from_point - self.translation;
        -GRAVITY * self.mass / displacement.length_squared() * displacement.normalize_or_zero()
    }
}

#[derive(bevy::prelude::Resource, Debug, Default)]
pub struct GravityField {
    pub masses: Vec<Mass>,
}

impl GravityField {
    pub fn acceleration_at(&self, point: Vec2) -> Vec2 {
        self.masses
            .iter()
            .map(|mass| mass.acceleration_to(point))
            .sum()
    }

    pub fn trajectory_starting_at(
        &self,
        start: crate::TrajectoryNode,
        delta_secs: f32,
    ) -> crate::Trajectory {
        crate::Trajectory {
            state: start,
            gravity: self,
            mass: 1.0,
            delta_secs,
        }
    }
}

impl FromIterator<Mass> for GravityField {
    fn from_iter<T: IntoIterator<Item = Mass>>(iter: T) -> Self {
        Self {
            masses: iter.into_iter().collect(),
        }
    }
}
