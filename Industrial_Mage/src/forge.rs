// This should hold all systems to do with Ore processing and Smelting
use bevy::prelude::*;

// Should Forge hold all the spawning of buildings? 


#[derive(Component, Debug)]
struct techlevel{
    speed: f32,
    level: f32,
    experience: f32,
    maxlevel: f32,
}


pub fn spawn_wizard_tower(){}

pub fn spawn_forge(){}