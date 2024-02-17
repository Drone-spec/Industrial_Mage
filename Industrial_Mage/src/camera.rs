// This should have all logic that details out Goblins and Goblin behavior.
use crate::wizard::*;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*, transform,
    //sprite::MaterialMesh2dBundle,
};
//use bevy_pancam::{PanCam, /*PanCamPlugin*/};

#[allow(dead_code)]
pub struct CameraLogic;

impl Plugin for CameraLogic {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, setupcamera);
    app.add_systems(Update, follow_player);
    }
}



fn setupcamera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()},
        BloomSettings{
            ..default()}, // 3. Enable bloom for the camera
    ));//.insert(PanCam::default());
}

fn follow_player(
    playerloc: Query<&Transform, With<Wizard>>,
    mut cameraloc: Query<&mut Transform, (With<Camera>, Without<Wizard>)>,
){
    for ( wizard_loc) in &playerloc {
        let pos = wizard_loc.translation;
        for mut transforms in &mut cameraloc {
            transforms.translation.x = pos.x;
            transforms.translation.y = pos.y;
        }
    }
}