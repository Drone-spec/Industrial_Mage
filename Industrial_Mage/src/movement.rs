
use bevy::{ecs::component, prelude::*};
use noise::core::value;
//Movement should all be handled here
pub struct MovementPlugin;
#[allow(dead_code)]

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Update, (update_position, update_velocity));
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity 
{
    pub fn new(value: Vec3) -> Self {Self {value}}
}
    

#[derive(Component, Debug)]
pub struct Acceleration
{
    pub value: Vec3,
}

impl Acceleration
{
    pub fn new(value: Vec3) -> Self {Self {value}}
}

#[derive(Bundle)]

pub struct MovingObjectBundle
{
   pub velocity: Velocity,
   pub acceleration: Acceleration,
   pub model: SpriteBundle,
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) 
{
    for (acceleration, mut velocity) in query.iter_mut()
    {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}


pub fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>)
{
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
        
    }
}

#[derive(Component,Debug)]
pub struct MoveToWizard;

