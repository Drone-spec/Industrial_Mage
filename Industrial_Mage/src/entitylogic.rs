use bevy::ecs::bundle;
// This should have all logic that details out Goblins and Goblin behavior.
use bevy::prelude::*;
use crate::wizard::*;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
pub struct EntityLogic;

impl Plugin for EntityLogic {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_goblin);
    }
}


#[derive(Component, Debug)]
pub struct Health {
    pub amount: f64,
    pub max: f64,
    pub regen: bool,
    pub regenamount: f64,
}


#[derive(Component, Debug)]
pub struct Mana {
    pub amount: f64,
    pub max: f64,
    pub regen: bool,
    pub regenamount: f64,
}


#[derive(Component, Debug)]
pub struct Goblin;


#[derive(Bundle)]
struct BasicGoblin {
    health: Health,
    mana: Mana,
    movingobjbun: MovingObjectBundle,
}



// This will be the command for bringing more goblins into the fold, world
fn spawn_goblin(mut commands: Commands, asset_server: Res<AssetServer> ){


    commands.spawn((BasicGoblin 
        {
            health: Health{
                amount: 100.0,
                max: 100.0,
                regen: true,
                regenamount: 0.0
            },
            mana: Mana{
                amount: 100.0,
                max: 100.0,
                regen: true,
                regenamount: 5.0
            },
            movingobjbun: MovingObjectBundle {
                velocity: Velocity::new(Vec3::ZERO),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SpriteBundle{
                    texture: asset_server.load("goblinhole/goblin.png"),
                    transform: Transform::from_translation(STARTING_TRANSLATION),
                    ..default()
                }
            },
        },
    ));
}


// This will spawn a home which will increase the amount of goblins
fn spawn_goblinhome(){

}
