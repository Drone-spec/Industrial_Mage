use std::arch::x86_64::_MM_FLUSH_ZERO_OFF;

use bevy::{math::vec3, prelude::*, transform, ui::update};
use crate::movement::{self, *};

const STARTING_TRANSLATION: Vec3 = Vec3::new(500., 0., 4.);
const WIZARD_SPEED: f32 = 250.0;

// All Wizard stuff should be in here..
pub struct WizardPlugin;

#[derive(Component, Debug)]
pub struct Wizard;

impl Plugin for WizardPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_wizard)
        .add_systems(Update, wizard_movement_controls);
    }
}

pub fn spawn_wizard(mut commands: Commands, asset_server: Res<AssetServer> )
 {
    commands.spawn((MovingObjectBundle 
        {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SpriteBundle
            {
                texture: asset_server.load("wiznerd/Wizard.png"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
    Wizard,
    ));
}

fn wizard_movement_controls(mut query: Query<(&mut Transform, &mut Velocity), With<Wizard>>, keyboard_input: Res<Input<KeyCode>>) 
{
    let(transform, mut velocity) = query.single_mut();

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
        velocity.value = transform.left() * WIZARD_SPEED;
    } 
    else if keyboard_input.pressed(KeyCode::D)
    {
        velocity.value = transform.right() * WIZARD_SPEED;
    }
    else
    {
        velocity.value = transform.forward();
    }

}
