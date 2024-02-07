// This should hold all Logic to do with Ore and Maybe all functions associated with those systems
// Also World Generation!!!
use bevy::{prelude::*, render::texture, transform::commands};

pub struct OreLogicPlugin;

impl Plugin for OreLogicPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, genesis);
    }
}

// Below was built using the 2d Sprite Sheets demo from assets


const SPRITESHEET_HEIGHT: usize = 640;
const SPRITESHEET_WIDTH: usize = 640;
const TEXTURE_PATH: &str = "terra/terrain.png";

fn genesis(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(TEXTURE_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITESHEET_WIDTH as f32, SPRITESHEET_HEIGHT as f32), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },

    ));


}

