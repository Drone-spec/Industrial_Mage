use bevy::{prelude::*, transform};

use crate::wizard::*;


#[allow(dead_code)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Update, debug_print_position);
    }
}


pub fn debug_print_position(query: Query<(Entity, &Transform, With<Wizard>)>) {

    for (Entity, transform, wizard) in query.iter() {
        info!("Entity: {:?} is at Position {:?}", Entity, transform.translation);
        
    }
}
