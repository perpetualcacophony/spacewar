use bevy::math::Vec2;
pub const GRAVITATIONAL_CONSTANT: f32 = 6.6743e-11; // m^3 / kg s^2

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mass {
    pub translation: Vec2,
    pub mass: f32, // in kg
}

impl Mass {
    fn acceleration_to(&self, from_point: Vec2) -> Vec2 {
        let displacement = from_point - self.translation; // m
        -GRAVITATIONAL_CONSTANT * self.mass / displacement.length_squared()
            * displacement.normalize_or_zero()
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
            timestep: delta_secs,
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
