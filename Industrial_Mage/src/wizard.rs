use bevy::prelude::*;

// All Wizard stuff should be in here..
pub struct WizardPlugin;
impl Plugin for WizardPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_wizard());
    }
}


pub fn spawn_wizard(mut commands: Commands) {
    commands.spawn([Position {x: 0.0, y: 0.0}, Velocity {x: 0.0, y: 0.0}])

}