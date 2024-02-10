// This should have all logic that details out Goblins and Goblin behavior.
use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_pancam::{PanCam, PanCamPlugin};

#[allow(dead_code)]

pub struct CameraLogic;

impl Plugin for CameraLogic {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, setupcamera);
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
    )).insert(PanCam::default());
}

