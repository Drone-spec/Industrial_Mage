use bevy::{ecs::entity, prelude::*};

#[derive(Component)]
pub struct Spell
{
    pub lifetime:f32,
    pub speed:f32,
    pub direction : Vec2,
}

pub fn update_spells(mut spell_query : Query<(&mut Spell, &mut Transform, Entity)>, time:Res<Time>, mut commands : Commands)
{
    for (mut spell, mut transform, entity) in spell_query.iter_mut()
    {
        spell.lifetime -= time.delta_seconds();
        let moving = spell.speed*spell.direction*time.delta_seconds();
        transform.translation += Vec3::new(moving.x,-moving.y,0.);
        if spell.lifetime <= 0.
        {
            commands.entity(entity).despawn();
        }
    }
}

pub struct SpellInfo
{
    pub translation : Vec2,
    pub entity: Entity,
}
