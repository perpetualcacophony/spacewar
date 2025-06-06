#[cfg(debug_assertions)]
#[allow(unused_imports, clippy::single_component_path_imports)]
use bevy_dylib;

use std::ops::DerefMut;

use bevy::prelude::*;
use spacewar::*;

mod star;
use star::Star;

mod ship;
use ship::Ship;

mod missile;
use missile::Missile;

type Transform = Transform2d;

mod debug_info;

mod keybinds;
pub use keybinds::{KeyBinds, KeyPair};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(transform2d::Plugin)
        .add_plugins(star::Plugin)
        .add_plugins(ship::Plugin)
        .add_plugins(missile::Plugin)
        .add_plugins(debug_info::Plugin)
        .init_resource::<GravityField>()
        .insert_resource(KeyBinds::default())
        .add_systems(Startup, (startup, respawn_ship))
        .add_systems(
            Update,
            (
                respawn_ship.run_if(|keys: Res<ButtonInput<KeyCode>>, keybinds: Res<KeyBinds>| {
                    keys.any_just_pressed(keybinds.reset())
                }),
                zoom,
            ),
        )
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(star::Bundle {
        star: Star { mass: 1.6e16 },
        transform: Transform::default(),
    });

    /*     commands.spawn(star::Bundle {
        star: Star { mass: 200_000.0 },
        transform: Transform::default().with_translation(-Vec2::X * 500.0),
    }); */
}

fn respawn_ship(mut commands: Commands, ship: Option<Single<Entity, With<Ship>>>) {
    if let Some(ship) = ship {
        commands.entity(*ship).despawn();
    }

    commands.spawn(ship::Bundle {
        transform: Transform::default().with_translation(Vec2::new(-1000.0, 500.0)),
        ship: Ship {
            sas: None,
            ..Default::default()
        },
    });
}

fn zoom(
    mut camera: Single<&mut Projection, With<Camera2d>>,
    keys: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBinds>,
) {
    let projection = match **camera.deref_mut() {
        Projection::Orthographic(ref mut projection) => projection,
        _ => unimplemented!(),
    };

    if keys.any_pressed(keybinds.zoom().map(KeyPair::left)) {
        projection.scale *= 1.025
    } else if keys.any_pressed(keybinds.zoom().map(KeyPair::right)) {
        projection.scale *= 0.975
    }
}
