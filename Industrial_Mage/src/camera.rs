// This should have all logic that details out Goblins and Goblin behavior.
use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraLogic;

impl Plugin for CameraLogic {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, setupcamera);
    }
}



fn setupcamera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default()).insert(PanCam::default());
}