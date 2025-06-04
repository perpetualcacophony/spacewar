use crate::{GravityField, Mass, Transform};
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Star {
    pub mass: f32,
}

#[derive(Clone, Debug, Resource, Bundle)]
struct StarSprite {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
}

impl FromWorld for StarSprite {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: Mesh2d(world.add_asset(Circle::new(1.0))),
            material: MeshMaterial2d(world.add_asset(Color::WHITE)),
        }
    }
}

#[derive(Clone, Debug, Bundle)]
pub struct Bundle {
    pub transform: Transform,
    pub star: Star,
}

fn spawn_stars(
    mut commands: Commands,
    stars: Query<Entity, Added<Star>>,
    mut components: Query<(&mut Transform, &Star)>,
    sprite: Res<StarSprite>,
    mut gravity: ResMut<GravityField>,
) {
    for entity in stars.iter() {
        let (mut transform, star) = components.get_mut(entity).unwrap();

        transform.scale = Vec2::splat(10.0);

        gravity.masses.push(Mass {
            translation: transform.translation,
            mass: star.mass,
        });

        commands.entity(entity).insert(sprite.clone());
    }
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSprite>()
            .add_systems(PostStartup, spawn_stars)
            .add_systems(FixedPostUpdate, spawn_stars);
    }
}
