//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;



mod forge;
mod ore;
mod movement;
mod wizard;
mod debug;
mod camera;
// Above loads the other files that will hold the functions and systems. We will need to use pub fn to call them outside of that function
// MORE TO Follow
use bevy::{prelude::*, window::close_on_esc};
use movement::MovementPlugin;
use ore::OreLogicPlugin;
use wizard::WizardPlugin;
use debug::DebugPlugin;
use camera::CameraLogic;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
<<<<<<< HEAD
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_wizard)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("goblinhole/goblin.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("goblinhole/goblin.png"),
            transform: Transform::from_xyz(150., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("goblinhole/goblin.png"),
            transform: Transform::from_xyz(80., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("goblinhole/goblinwore.png"),
            transform: Transform::from_xyz(380., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

fn spawn_wizard (mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("wiznerd/Wizard.png"),
            transform: Transform::from_xyz(-380., 0., 0.),
            ..default()
        },
        Direction::Down,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }
        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}
=======
        .add_plugins(WizardPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(OreLogicPlugin)
        .add_plugins(CameraLogic)
        .add_systems(Update, close_on_esc)
        .run();
}
>>>>>>> e64e86eb5714a8069b218536d4bf4305185f8234
