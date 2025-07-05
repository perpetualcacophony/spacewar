pub mod transform2d {
    use bevy::prelude::*;

    #[derive(Debug, Clone, Copy, PartialEq, Component)]
    pub struct Transform2d {
        pub translation: Vec2,
        pub z_layer: f32,
        pub rotation: Rot2,
        pub scale: Vec2,
    }

    impl Default for Transform2d {
        fn default() -> Self {
            Self {
                translation: Vec2::ZERO,
                z_layer: 0.0,
                rotation: Rot2::IDENTITY,
                scale: Vec2::ONE,
            }
        }
    }

    impl Transform2d {
        pub fn with_translation(mut self, value: Vec2) -> Self {
            self.translation = value;
            self
        }

        pub fn with_z_layer(mut self, value: f32) -> Self {
            self.z_layer = value;
            self
        }

        pub fn with_rotation(mut self, value: Rot2) -> Self {
            self.rotation = value;
            self
        }

        pub fn with_scale(mut self, value: Vec2) -> Self {
            self.scale = value;
            self
        }

        pub fn local_y(&self) -> Dir2 {
            // Rot2 * unit vector is length 1
            Dir2::new_unchecked(self.rotation * Vec2::Y)
        }

        pub fn local_x(&self) -> Dir2 {
            // Rot2 * unit vector is length 1
            Dir2::new_unchecked(self.rotation * Vec2::X)
        }
    }

    fn transform2d_to_bevy_transform(
        mut query: Query<(&Transform2d, &mut Transform), Changed<Transform2d>>,
    ) {
        for (transform2d, mut bevy_transform) in query.iter_mut() {
            bevy_transform.translation = transform2d.translation.extend(transform2d.z_layer);
            bevy_transform.rotation = Quat::from_rotation_z(transform2d.rotation.as_radians());
            bevy_transform.scale = transform2d.scale.extend(1.0);
        }
    }

    pub struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(&self, app: &mut App) {
            app.add_systems(PostStartup, transform2d_to_bevy_transform)
                .add_systems(FixedPostUpdate, transform2d_to_bevy_transform);
        }
    }
}
pub use transform2d::Transform2d;

pub mod trajectory;
pub use trajectory::{Trajectory, TrajectoryNode};

pub mod gravity;
pub use gravity::{GravityField, Mass};

pub fn smoothstep(x: f32) -> f32 {
    3.0 * x.powi(2) - 2.0 * x.powi(3)
}

#[derive(Debug, Clone, Copy, PartialEq, bevy::prelude::Component, Default)]
pub struct OrbitalElements {
    pub periapsis: f32,
    pub eccentricity: f32,
}

impl OrbitalElements {
    pub const ZERO: Self = Self {
        periapsis: 0.0,
        eccentricity: 0.0,
    };
}
