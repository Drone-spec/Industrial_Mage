//use std::arch::x86_64::_MM_FLUSH_ZERO_OFF;

//use bevy::{/*input::mouse, math::vec3,*/ ecs::system::EntityCommands, math::vec2, prelude::* /*transform, ui::update*/};
use bevy::prelude::*;
use crate::movement::*/* , player_attach::{PlayerAttach,self}}{self, *}*/;

pub const STARTING_TRANSLATION  : Vec3 = Vec3::new(500., 0., 1.);
const WIZARD_SPEED              : f32  = 250.0;
const SPELL_SPEED               : f32  = 500.0;
const SPELL_FORWARD_SPAWN_SCALAR: f32  =  30.0;

// All Wizard stuff should be in here..
pub struct WizardPlugin;

#[derive(Component, Debug)]
pub struct Wizard;

#[derive(Component, Debug)]
pub struct WizardSpell;

impl Plugin for WizardPlugin 
{
    fn build(&self, app: &mut App) 
    {
        app.add_systems(Startup, spawn_wizard)
            .add_systems(Update, (wizard_movement_controls, wizard_weapon_controls),);
    }
}

pub fn spawn_wizard(mut commands: Commands, asset_server: Res<AssetServer> )
 {
    commands.spawn((MovingObjectBundle 
        {
            velocity    : Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model       : SpriteBundle
            {
                texture  : asset_server.load("wiznerd/Wizard.png"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
    Wizard,
    ));
}

fn wizard_movement_controls(mut query: Query<(&mut Transform, &mut Velocity), With<Wizard>>, keyboard_input: Res<Input<KeyCode>>) 
{
    let(mut transform, mut velocity) = query.single_mut();

        if keyboard_input.pressed(KeyCode::S)
    {
        velocity.value = transform.down() * WIZARD_SPEED;
    } 
    else if keyboard_input.pressed(KeyCode::W)
    {
        velocity.value = transform.up() * WIZARD_SPEED;
    }
    else if keyboard_input.pressed(KeyCode::A)
    {
        // Rotation changes which way the sprite faces.
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        velocity.value = transform.right() * WIZARD_SPEED;
    } 
    else if keyboard_input.pressed(KeyCode::D)
    {
        // Rotation is to tell the Sprite which way to face~!
        transform.rotation = Quat::default();
        velocity.value = transform.right() * WIZARD_SPEED;
    }
    else
    {
        velocity.value = transform.back() + transform.forward();
    }
}

fn wizard_weapon_controls(mut commands: Commands, query: Query<&Transform, With<Wizard>>, keyboard_input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>,)
{
    let transform = query.single();

    if keyboard_input.pressed(KeyCode::Space)
    {
        commands.spawn((MovingObjectBundle
        {
            velocity    : Velocity::new(transform.right() * SPELL_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            model       : SpriteBundle
            {
                texture: asset_server.load("wiznerd/fireball.png"),
                transform: Transform::from_translation(transform.translation + transform.right() * SPELL_FORWARD_SPAWN_SCALAR),
                ..default()
            },
        },
        WizardSpell,
        ));
    }
}


