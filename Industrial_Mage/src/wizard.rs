use bevy::prelude::*;
use crate::movement::*;

// All Wizard stuff should be in here..
pub struct WizardPlugin;
impl Plugin for WizardPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_wizard);
    }
}


pub fn spawn_wizard(mut commands: Commands, asset_server: Res<AssetServer> ) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("wiznerd/Wizard.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Velocity {
            value: Vec3::new(0., 0., 0.)
        }
    ));


}