use crate::{GravityField, KeyBinds, KeyPair, Transform};
use bevy::prelude::*;
use spacewar::TrajectoryNode;

#[derive(Component, Clone, Debug)]
pub struct Ship {
    pub velocity: Vec2,
    pub rotational_velocity: f32,
    pub sas: Option<SASMode>,
    pub draw_trajectory: usize,
    pub trajectory_gap: usize,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            velocity: Vec2::X * 300.0,
            rotational_velocity: 0.0,
            sas: Some(SASMode::default()),
            draw_trajectory: 500,
            trajectory_gap: 10,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SASMode {
    #[default]
    Stability,
    Prograde,
    Retrograde,
}

#[derive(Clone, Debug, Bundle, Default)]
pub struct Bundle {
    pub ship: Ship,
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
            mesh: Mesh2d(world.add_asset(Triangle2d::new(
                -Vec2::Y / 3.0 - Vec2::X / 3.0,
                Vec2::X - Vec2::X / 3.0,
                Vec2::Y / 3.0 - Vec2::X / 3.0,
            ))),
            material: MeshMaterial2d(world.add_asset(Color::WHITE)),
        }
    }
}

fn spawn_ships(
    mut commands: Commands,
    stars: Query<Entity, Added<Ship>>,
    mut components: Query<&mut Transform>,
    sprite: Res<Sprite>,
) {
    for entity in stars.iter() {
        let mut transform = components.get_mut(entity).unwrap();

        transform.scale = Vec2::splat(30.0);

        commands
            .entity(entity)
            .insert(sprite.clone())
            .insert(trail::Trail::default());
    }
}

fn change_speed(
    ship: Single<(&mut Ship, &Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBinds>,
) {
    let (mut ship, transform) = ship.into_inner();

    if keys.any_pressed(keybinds.accelerate()) {
        ship.velocity += 0.015 * transform.local_x();
    }
}

fn change_target(mut ship: Single<&mut Ship>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::KeyP) {
        if let Some(sas) = ship.sas.as_mut() {
            if *sas == SASMode::Prograde {
                *sas = SASMode::Stability;
            } else {
                *sas = SASMode::Prograde;
            }
        }
    } else if keys.just_pressed(KeyCode::KeyT) {
        if ship.sas.is_none() {
            ship.sas = Some(SASMode::Stability);
        } else {
            ship.sas = None;
        }
    }
}

fn change_angle(
    ship: Single<(&mut Ship, &Transform), With<Ship>>,
    keys: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBinds>,
) {
    let (mut ship, transform) = ship.into_inner();

    if keys.any_pressed(keybinds.rotation_speed().map(KeyPair::left)) {
        ship.rotational_velocity += 1.0;
    } else if keys.any_pressed(keybinds.rotation_speed().map(KeyPair::right)) {
        ship.rotational_velocity -= 1.0;
    }

    if let Some(sas) = ship.sas {
        match sas {
            SASMode::Stability => ship.rotational_velocity -= 0.03 * ship.rotational_velocity,
            SASMode::Prograde | SASMode::Retrograde => {
                let mut target_heading = Rot2::radians(ship.velocity.to_angle());
                if sas == SASMode::Retrograde {
                    target_heading *= Rot2::PI;
                };

                let delta =
                    target_heading.as_turn_fraction() - transform.rotation.as_turn_fraction();
                ship.rotational_velocity += 0.16 * delta;
            }
        }
    }
}

fn update_ship(
    ship: Single<(&mut Ship, &mut Transform), With<Ship>>,
    gravity: Res<GravityField>,
    time: Res<Time>,
) {
    let (mut ship, mut ship_transform) = ship.into_inner();

    ship_transform.rotation *= Rot2::degrees(ship.rotational_velocity * time.delta_secs());

    let mut trajectory = gravity.trajectory_starting_at(
        TrajectoryNode::from_translation_velocity(ship_transform.translation, ship.velocity),
        time.delta_secs(),
    );

    let next_node = trajectory.next().unwrap();

    ship.velocity = next_node.velocity();
    ship_transform.translation = next_node.translation();
}

fn draw_trajectory(
    ships: Query<(&Ship, &Transform)>,
    gravity: Res<GravityField>,
    mut gizmos: Gizmos,
    time: Res<Time<Fixed>>,
) {
    for (ship, transform) in ships.iter() {
        let trajectory = gravity.trajectory_starting_at(
            TrajectoryNode::from_translation_velocity(transform.translation, ship.velocity),
            time.delta_secs(),
        );

        for (i, node) in trajectory
            .step_by(ship.trajectory_gap)
            .take(ship.draw_trajectory)
            .enumerate()
        {
            gizmos.circle_2d(
                node.translation(),
                1.0,
                Color::oklch(1.0, 0.8, 240.0)
                    .with_alpha(1.0 - i as f32 / ship.draw_trajectory as f32),
            );
        }
    }
}

fn fire_missile(mut commands: Commands, ship: Single<(&Ship, &Transform)>) {
    commands.spawn(crate::missile::Bundle {
        transform: *ship.1,
        missile: crate::Missile {
            speed: ship.0.velocity.length() + 2.0,
            ..Default::default()
        },
    });
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprite>()
            .add_systems(PostStartup, spawn_ships)
            .add_systems(
                Update,
                (
                    change_target,
                    change_speed,
                    change_angle,
                    trajectory_drawing_keybinds,
                    fire_missile.run_if(bevy::input::common_conditions::input_just_pressed(
                        KeyCode::Space,
                    )),
                ),
            )
            .add_systems(FixedUpdate, (update_ship, trail::define_trail))
            .add_systems(
                PostUpdate,
                (trail::draw_trail, draw_trajectory, spawn_ships),
            );
    }
}

pub fn trajectory_drawing_keybinds(
    mut ship: Single<&mut Ship>,
    keys: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBinds>,
) {
    let shift = keys.pressed(KeyCode::ShiftLeft);

    if keys.any_pressed(keybinds.trajectory_length().map(KeyPair::shorter)) {
        if shift {
            ship.trajectory_gap = ship.trajectory_gap.saturating_sub(2)
        } else {
            ship.draw_trajectory = ship.draw_trajectory.saturating_sub(5)
        }
    } else if keys.any_pressed(keybinds.trajectory_length().map(KeyPair::longer)) {
        if shift {
            ship.trajectory_gap = ship.trajectory_gap.saturating_add(2)
        } else {
            ship.draw_trajectory = ship.draw_trajectory.saturating_add(5)
        }
    }

    if ship.trajectory_gap == 0 {
        ship.trajectory_gap = 1
    }
}

mod trail {
    const TRAIL_LENGTH: usize = 10;

    use super::Transform;
    use bevy::prelude::*;
    use std::collections::VecDeque;

    #[derive(Component, Clone, Debug)]
    pub struct Trail(VecDeque<Vec2>);

    impl Default for Trail {
        fn default() -> Self {
            Self(VecDeque::with_capacity(TRAIL_LENGTH))
        }
    }

    pub fn define_trail(mut trails: Query<(&mut Trail, &Transform)>) {
        for (mut trail, transform) in trails.iter_mut() {
            while trail.0.len() >= TRAIL_LENGTH {
                trail.0.pop_back();
            }
            trail.0.push_front(transform.translation);
        }
    }

    pub fn draw_trail(trails: Query<&Trail>, mut gizmos: Gizmos) {
        for trail in trails {
            for (i, node) in trail.0.iter().enumerate() {
                gizmos.circle_2d(
                    *node,
                    1.0,
                    Color::oklch(1.0, 0.8, 0.0).with_alpha(1.0 - i as f32 / TRAIL_LENGTH as f32),
                );
            }
        }
    }
}
