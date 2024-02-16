use bevy::{ecs::bundle, transform};
use std::*;
// This should have all logic that details out Goblins and Goblin behavior.
use bevy::prelude::*;
use crate::wizard::*;
#[allow(dead_code)]
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
pub struct EntityLogic;

#[derive(Resource)]
pub struct HealthRegenTimer(Timer);
#[derive(Resource)]
pub struct ManaRegenTimer(Timer);


impl Plugin for EntityLogic {
    fn build(&self, app: &mut App) {
    app.insert_resource(HealthRegenTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
    .insert_resource(ManaRegenTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
    .add_systems(Startup, spawn_goblin)
    .add_systems(FixedUpdate, health_regen)
    .add_systems(Update, mana_regen)
    .add_systems(Update, enemy_update)
    ;
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


#[derive(Component, Debug)]
pub struct Building;


#[derive(Component, Debug)]
pub struct House;

#[derive(Component, Debug)]
pub struct Defense;

#[derive(Component, Debug)]
pub struct Enemy{
    speed: f32,
}

#[derive(Bundle)]
struct BasicGoblin {
    health: Health,
    mana: Mana,
    movingobjbun: MovingObjectBundle,
}







/// The Health and Mana system work the exact same!!
/// It just Query's for every Entity with Health and the pull Time and the Resource time.
/// If Resource time has passed its value then run the below System That system will see if Regen is enabled and then ask the entity
/// what its regen amount is and apply that to the main amount for that entity.
fn health_regen(mut query: Query<(Entity, &mut Health)>, time: Res<Time>, mut timer: ResMut<HealthRegenTimer>){
    if timer.0.tick(time.delta()).just_finished() {
        for (Entity, mut Health) in query.iter_mut()
        {   
            match Health.regen {
                true =>
                if Health.max > Health.amount {
                    Health.amount += Health.regenamount + (time.delta_seconds() * 2.0);
                    info!("Entity: {:?} is at {:.2}% Health!", Entity, ((Health.amount / Health.max) * 100.00));
                }
                else 
                {
                    info!("Entity: {:?} is at has max Health! {:.2}", Entity, ((Health.amount / Health.max) * 100.00));
                },
                false => 
                if Health.max > Health.amount {
                    info!("Entity: {:?} is at {:.2} Health and Regen is hindered!", Entity, ((Health.amount / Health.max) * 100.00));
                }   
                else
                {
                    info!("Entity: {:?} is at {:.2} Health! Regen is hindered", Entity, ((Health.amount / Health.max) * 100.00));
                },
                _ => info!("How did you get here? Error at True False in health_regen") 
            }
        }
    }
}

#[allow(dead_code)]
fn mana_regen(mut query: Query<(Entity, &mut Mana)>, time: Res<Time>, mut timer:ResMut<ManaRegenTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (Entity, mut Mana) in query.iter_mut()
        {
            match Mana.regen {
                true =>
                if Mana.max > Mana.amount {
                    Mana.amount += Mana.regenamount + (time.delta_seconds() * 2.0);
                    info!("Entity: {:?} is at {:.2} Mana!", Entity, Mana.amount);
                }
                else 
                {
                    info!("Entity: {:?} is at has max Mana! {:.2}", Entity, Mana.amount);
                },
                false => 
                if Mana.max > Mana.amount {
                    info!("Entity: {:?} is at {:.2} Mana and Regen is hindered!", Entity, Mana.amount);
                }   
                else
                {
                    info!("Entity: {:?} is at {:.2} Mana! Regen is hindered", Entity, Mana.amount);
                },
                _ => info!("How did you get here? Error at True False in Mana_regen") 
            }
        }
    }
}
 



// This will be the command for bringing more goblins into the fold, world
fn spawn_goblin(mut commands: Commands, asset_server: Res<AssetServer> ){


    commands.spawn((BasicGoblin 
        {
            health: Health{
                amount: 50.0,
                max: 100.0,
                regen: true,
                regenamount: 3.0
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
    commands.spawn((
        
        BasicGoblin 
        {
            health: Health{
                amount: 50.0,
                max: 100.0,
                regen: true,
                regenamount: 2.0
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
    commands.spawn((BasicGoblin 
        {
            health: Health{
                amount: 50.0,
                max: 300.0,
                regen: true,
                regenamount: 10.0
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
                    texture: asset_server.load("goblinhole/evilgoblin.png"),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                    ..default()
                }
            },
        },
        // This looks like shit but maybe we can improve it later...
        Enemy{speed: 50.0}
    ));
}


// This will spawn a home which will increase the amount of goblins
// Maybe this should also be a build item for the wizard?
fn spawn_goblinhome(){

}

// Attempting to follow a guide and make it so the creatures follow and die tooo.
fn enemy_update(time:Res<Time>,
    mut find_enemy:Query<(&Enemy, &mut Transform,Entity,&Health),Without<Wizard>>,
    mut find_wizard:Query<(&Wizard, &mut Transform),Without<Enemy>>,
    mut commands: Commands) {
        if let Ok((_wizard_movement,wizard_transform)) = find_wizard.get_single()
        {
            for(enemy, mut transform,entity,health) in find_enemy.iter_mut()
            {
                let moving = Vec3::normalize(wizard_transform.translation-transform.translation)*enemy.speed*time.delta_seconds();
                transform.translation+=moving;

                if health.amount <=0.
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }