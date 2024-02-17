use bevy::prelude::*;

use crate::{wizard::Wizard};

#[derive(Component)]
pub struct PlayerAttach
{
    pub offset : Vec2,
}

pub fn attach_objects(player_query:Query<(&Wizard, &mut Transform), Without<PlayerAttach>>, mut objects_query: Query<(&PlayerAttach,&mut Transform), Without<Wizard>>)
{
    if let Ok((movement_data,player_transform)) = player_query.get_single()
    {
        for(attach, mut transform) in objects_query.iter_mut()
        {
            transform.translation = player_transform.translation+Vec3::new(attach.offset.x,attach.offset.y,1.);
        }
    }
} 