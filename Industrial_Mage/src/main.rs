//! Renders a 2D scene containing a single, moving sprite.

//mod forge;
mod ore;
mod movement;
mod wizard;
mod debug;
mod camera;
mod entitylogic;
pub mod animation;
pub mod player_attach;
pub mod staff;
pub mod cursor_info;
pub mod wizard_spell;
//mod despawn;
// Above loads the other files that will hold the functions and systems. We will need to use pub fn to call them outside of that function
// MORE TO Follow
use bevy       ::{prelude::*, window::close_on_esc};
use std        ::collections::HashMap;
use bevy_pancam::PanCamPlugin;
use movement   ::MovementPlugin;
use ore        ::OreLogicPlugin;
use wizard     ::WizardPlugin;
use debug      ::DebugPlugin;
use camera     ::CameraLogic;
use entitylogic::EntityLogic;
//use despawn::DespawnPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WizardPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(OreLogicPlugin)
        .add_plugins(CameraLogic)
        .add_plugins(PanCamPlugin)
        //.add_plugins(DespawnPlugin)
        .add_plugins(EntityLogic)
        .add_systems(Update,
             (close_on_esc, animation::animate_sprite, player_attach::attach_objects,
                       staff::staff_controls, wizard_spell::update_spells))
        .add_systems(Startup, setup)
        .insert_resource(Msaa::Off)
        .insert_resource(cursor_info::OffsetedCursorPostion{x:0.,y:0.})
        .run();
}


pub fn create_staff_anim_hashmap() -> HashMap<String,animation::Animation>
{
    let mut hash_map = HashMap::new();

    hash_map.insert("Shoot".to_string(), animation::Animation{start:1,end:5,looping:false,cooldown:0.1}); // something is wrong with the png sprite

    hash_map.insert("Idle".to_string(), animation::Animation{start:1,end:1,looping:true,cooldown:0.1});

    return hash_map;
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases : ResMut<Assets<TextureAtlas>>)
{
    let texture_handle = asset_server.load("wiznerd/staff-sheet.png");
    let texture_atlas = TextureAtlas::from_grid
    (
        texture_handle,
        Vec2::new(16.0, 16.0),
        5,
        1,
        None, 
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    //commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteSheetBundle
    {
        texture_atlas:texture_atlas_handle,
        transform:Transform::from_scale(Vec3::splat(2.0)),
        // This above from_scale is what handled the SIZE of the sprite.
        ..default()
    }) 
    .insert(animation::Animator
    {
        timer:0.,
        cooldown:0.05,
        last_animation: "Shoot".to_string(),
        current_animation: "Shoot".to_string(),
        animation_bank: create_staff_anim_hashmap(),
    })
    .insert(player_attach::PlayerAttach{offset:Vec2::new(20.,-15.)}).insert(staff::StaffController{shoot_cooldown:0.3, shoot_timer:0.});
}