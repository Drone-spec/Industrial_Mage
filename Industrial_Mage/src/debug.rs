use bevy::prelude::*;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Update, debug_print_position);
    }
}


pub fn debug_print_position(query: Query<Entity, &Position>) {

    for (entity, position) in query.iter() {
        info!("Entity: {:?} is at Position {:?}", entity, position);
    }
}
