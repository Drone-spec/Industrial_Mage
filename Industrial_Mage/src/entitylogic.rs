use bevy::ecs::bundle;
use std::*;
// This should have all logic that details out Goblins and Goblin behavior.
use bevy::prelude::*;
use crate::wizard::*;
#[allow(dead_code)]
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
pub struct EntityLogic;

impl Plugin for EntityLogic {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_goblin)
    .add_systems(Update, (health_regen, mana_regen));
    }
}


#[derive(Component, Debug)]
pub struct Health {
    pub amount: f32,
    pub max: f32,
    pub regen: bool,
    pub regenamount: f32,
}


#[derive(Component, Debug)]
pub struct Mana {
    pub amount: f32,
    pub max: f32,
    pub regen: bool,
    pub regenamount: f32,
}


#[derive(Component, Debug)]
pub struct Goblin;


#[derive(Bundle)]
struct BasicGoblin {
    health: Health,
    mana: Mana,
    movingobjbun: MovingObjectBundle,
}
fn health_regen(mut query: Query<(Entity, &mut Health)>, time: Res<Time>){
    
    for (Entity, mut Health) in query.iter_mut()
    {
        if Health.regen == true {
            Health.amount += Health.regenamount * time.delta_seconds ();
            info!("Entity: {:?} is at has {:?} Health!", Entity, Health.amount);
        } 
        
    }

}

#[allow(dead_code)]
fn mana_regen(mut query: Query<(Entity, &mut Mana)>, time: Res<Time>) {
    
    for (Entity, mut Mana) in query.iter_mut()
    {
        if Mana.regen == true {
            Mana.amount += Mana.regenamount * time.delta_seconds ();
            info!("Entity: {:?} is at has {:?} Mana!", Entity, Mana.amount);
        } 
        
    }
} 



// This will be the command for bringing more goblins into the fold, world
fn spawn_goblin(mut commands: Commands, asset_server: Res<AssetServer> ){


    commands.spawn((BasicGoblin 
        {
            health: Health{
                amount: 100.0,
                max: 100.0,
                regen: true,
                regenamount: 1.0
            },
            mana: Mana{
                amount: 0.0,
                max: 100.0,
                regen: true,
                regenamount: 1.0
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
