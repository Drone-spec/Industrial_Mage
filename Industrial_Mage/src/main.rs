
mod forge;
mod ore;
mod movement;
mod wizard;
mod debug;
mod camera;
// Above loads the other files that will hold the functions and systems. We will need to use pub fn to call them outside of that function
// MORE TO Follow
use bevy::prelude::*;
use movement::MovementPlugin;
use wizard::WizardPlugin;
use debug::DebugPlugin;
use camera::CameraLogic;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WizardPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraLogic)
        .run();
}
