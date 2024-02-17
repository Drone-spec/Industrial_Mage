use bevy::{math::vec3, prelude::*, window::PrimaryWindow};

use crate::{animation::Animator, cursor_info::OffsetedCursorPostion, player_attach, wizard::Wizard, wizardSpell::Spell};

const SPELL_LIFETIME: f32 = 0.5;
const SPELL_SPEED   : f32 = 1000.;

#[derive(Component)]
pub struct StaffController
{
    pub shoot_cooldown: f32,
    pub shoot_timer   : f32,
}

pub fn staff_controls(mut cursor_res:ResMut<OffsetedCursorPostion>, 
    mut staff_query : Query<(&mut StaffController, &mut Transform, &mut Animator)>,
    mut cursor:EventReader<CursorMoved>,primary_query:Query<&Window,With<PrimaryWindow>>,
    time : Res<Time>,
    buttons:Res<Input<MouseButton>>,
    asset_server : Res<AssetServer>,
    mut commands : Commands)
    {
        for(mut staff_controller, mut transform, mut animator) in staff_query.iter_mut()
        {
            staff_controller.shoot_timer-=time.delta_seconds();
            if staff_controller.shoot_timer > 0.
            {
                animator.current_animation = "Shoot".to_string();
            }
            else
            {
                animator.current_animation = "Idle".to_string();
            }
            let Ok(primary) = primary_query.get_single() else
            {
                return;
            };
            let mut cursor_position = match cursor.read().last()
            {
                Some(cursor_moved) => cursor_moved.position,
                None => Vec2::new(cursor_res.x+primary.width()/2.,cursor_res.y+primary.height()/2.),
            };

            cursor_position.x -= primary.width()/2.;
            cursor_position.y -= primary.height()/2.;

            cursor_res.x = cursor_position.x;
            cursor_res.y = cursor_position.y;

            let diff = cursor_position - Vec2::new(transform.translation.x,transform.translation.y);
            let angle = -diff.y.atan2(diff.x);
            transform.rotation = Quat::from_axis_angle(Vec3::new(0.,0.,1.),angle);

            if staff_controller.shoot_timer <= 0.
            {
                if buttons.pressed(MouseButton::Left)
                {
                    let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
                    spawn_transform.translation = transform.translation;// + Vec3::new(0., 15., 0.);
                    // ^^^^ this changes where the object spwans by adding the vec3 to it
                    spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0.,0.,1.), angle);
                    staff_controller.shoot_timer = staff_controller.shoot_cooldown;
                    commands.spawn(SpriteBundle
                    {
                        transform:spawn_transform,
                        texture:asset_server.load("wiznerd/fireball.png"),
                        ..default()
                    }).insert(Spell{lifetime:SPELL_LIFETIME,speed:SPELL_SPEED,direction:diff.normalize()});//.insert(player_attach::PlayerAttach{offset:Vec2::new(0.,20.)});
                }
            }
        }
    }