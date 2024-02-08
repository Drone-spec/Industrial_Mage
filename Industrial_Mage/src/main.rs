//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;



mod forge;
mod ore;
mod movement;
mod wizard;
mod debug;
mod camera;
//mod despawn;
// Above loads the other files that will hold the functions and systems. We will need to use pub fn to call them outside of that function
// MORE TO Follow
use bevy::{prelude::*, window::close_on_esc};
use bevy_pancam::PanCamPlugin;
use movement::MovementPlugin;
use ore::OreLogicPlugin;
use wizard::WizardPlugin;
use debug::DebugPlugin;
use camera::CameraLogic;
//use despawn::DespawnPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WizardPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(OreLogicPlugin)
        .add_plugins(CameraLogic)
        .add_plugins(PanCamPlugin)
        //.add_plugins(DespawnPlugin)
        .add_systems(Update, close_on_esc)
        .insert_resource(Msaa::Off)
        .run();
}
