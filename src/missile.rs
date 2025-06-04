use crate::{GravityField, Transform};
use bevy::prelude::*;
use spacewar::TrajectoryNode;

#[derive(Clone, Debug, Component)]
pub struct Missile {
    pub speed: f32,
    pub age: f32,
}

impl Default for Missile {
    fn default() -> Self {
        Self {
            speed: 5.0,
            age: 0.0,
        }
    }
}

#[derive(Clone, Debug, Bundle, Default)]
pub struct Bundle {
    pub missile: Missile,
    pub transform: Transform,
}

#[derive(Clone, Debug, Resource, Bundle)]
struct Sprite {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
}

impl FromWorld for Sprite {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: Mesh2d(world.add_asset(Circle::new(0.5))),
            material: MeshMaterial2d(world.add_asset(Color::WHITE)),
        }
    }
}

fn spawn(
    mut commands: Commands,
    stars: Query<Entity, Added<Missile>>,
    mut components: Query<&mut Transform>,
    sprite: Res<Sprite>,
) {
    for entity in stars.iter() {
        let mut transform = components.get_mut(entity).unwrap();

        transform.scale = Vec2::splat(10.0);

        commands.entity(entity).insert(sprite.clone());
    }
}

fn update_missile(mut missile: Query<(&mut Missile, &mut Transform)>, gravity: Res<GravityField>) {
    for (mut missile, mut missile_transform) in missile.iter_mut() {
        let mut trajectory = gravity.trajectory_starting_at(
            TrajectoryNode::from_translation_velocity(
                missile_transform.translation,
                missile.speed * missile_transform.local_x(),
            ),
            1.0,
        );

        trajectory.mass = 1.0;

        let velocity = trajectory.next_guaranteed().velocity();
        missile_transform.rotation = Rot2::radians(velocity.to_angle());
        missile.speed = velocity.length();

        let direction = missile_transform.local_x();
        missile_transform.translation += missile.speed * direction;
    }
}

fn update_age(mut missiles: Query<&mut Missile>, time: Res<Time>) {
    for mut missile in missiles.iter_mut() {
        missile.age += time.delta_secs();
    }
}

fn aging(
    mut commands: Commands,
    mut missiles: Query<(Entity, &Missile, &mut MeshMaterial2d<ColorMaterial>)>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    const MAX_AGE: f32 = 30.0;

    for (id, missile, mut material) in missiles.iter_mut() {
        if missile.age > MAX_AGE {
            commands.entity(id).despawn();
            continue;
        }

        *material =
            MeshMaterial2d(colors.add(Color::WHITE.with_alpha(1.0 - missile.age / MAX_AGE)));
    }
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprite>()
            .add_systems(PostStartup, spawn)
            .add_systems(FixedUpdate, (update_missile, update_age))
            .add_systems(PostUpdate, (spawn, aging));
    }
}
