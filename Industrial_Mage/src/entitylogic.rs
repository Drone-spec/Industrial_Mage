// This should have all logic that details out Goblins and Goblin behavior.
use bevy::prelude::*;
use crate::wizard::*;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
pub struct EntityLogic;

impl Plugin for EntityLogic {
    fn build(&self, app: &mut App) {
    //app.add_systems(Update, debug_print_position);
    }
}


#[derive(Component, Debug)]
pub struct Health {
    amount: f64,
    max: f64,
    regen: bool,
    regenamount: f64,
}


#[derive(Component, Debug)]
pub struct Mana {
    amount: f64,
    max: f64,
    regen: bool,
    regenamount: f64,
}


#[derive(Component, Debug)]
pub struct Goblin;




// This will be the command for bringing more goblins into the fold, world
fn spawn_goblin(mut commands: Commands, asset_server: Res<AssetServer> ){


    commands.spawn((MovingObjectBundle 
        {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SpriteBundle
            {
                texture: asset_server.load("golbinhole/goblin.png"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
    Wizard,
    ));
}


// This will spawn a home which will increase the amount of goblins
fn spawn_goblinhome(){

}
