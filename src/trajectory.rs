use crate::GravityField;
use bevy::math::Vec2;

pub struct Trajectory<'g> {
    pub gravity: &'g GravityField,
    pub state: TrajectoryNode,
    pub mass: f32,
    pub timestep: f32,
}

impl Trajectory<'_> {
    pub fn next_guaranteed(&mut self) -> TrajectoryNode {
        self.state.velocity += self.mass * self.gravity.acceleration_at(self.state.translation());

        self.state.translation += self.state.velocity() * self.timestep;
        self.state
    }

    pub fn eccentricity(&self) -> f32 {
        let standard_gravitational_parameter = crate::gravity::GRAVITATIONAL_CONSTANT * 1.6e16;

        let _angular_momentum = {
            let r = self.state.translation();
            let v = self.state.velocity();
            let theta = (-v).angle_to(-self.state.translation);

            r.length() * v.length() * f32::sin(theta)
        };

        // vis-viva equation
        let specific_orbital_energy = {
            let v = self.state.velocity().length();
            let r = self.state.translation().length();
            let mu = standard_gravitational_parameter;

            dbg!(v.powi(2) / 2.0);
            dbg!(-mu / r);

            v.powi(2) / 2.0 - mu / r
        };

        //dbg!(specific_orbital_energy);

        let specific_relative_angular_momentum = self
            .state
            .translation()
            .extend(0.0)
            .cross(self.state.velocity().extend(0.0))
            .z;

        (1.0 + (2.0 * specific_orbital_energy * specific_relative_angular_momentum.powi(2)
            / standard_gravitational_parameter.powi(2)))
        .sqrt()
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
