use bevy::app::Plugin;

//Movement should all be handled here
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Update, update_position);
    }
}

#[derive(Component, Debug)]
pub struct Position {
   pub x: f32,
   pub y: f32,

}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn update_position(mut query: Query<&Velocity, &mut Position>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}


