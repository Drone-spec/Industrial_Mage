// This should hold all Logic to do with Ore and Maybe all functions associated with those systems
// Also World Generation!!!
use bevy::{math::vec3, prelude::*, render::texture, transform::commands, utils::hashbrown::HashSet};
use noise::Perlin;

pub struct OreLogicPlugin;

impl Plugin for OreLogicPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, genesis);
    }
}

// Below was built using the 2d Sprite Sheets demo from assets

const MAPHIGHT: usize = 1000;
const MAPWIDTH: usize = 1000;
const SPRITESHEET_HEIGHT: usize = 64;
const SPRITESHEET_WIDTH: usize = 64;
const TEXTURE_PATH: &str = "terra/terrain.png";
const SPRITESCALE: usize = 5;

fn genesis(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // BELOW IS USED TO HANDLE THE LOADING OF MAP ASSETS
    let texture_handle = asset_server.load(TEXTURE_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITESHEET_WIDTH as f32, SPRITESHEET_HEIGHT as f32), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Below is the actual Generation of some of the map assets!
    let mut rngseed = rand::thread_rng();
    let perlin: Perlin = Perlin::new(rngseed.gen());
    

    let mut tiles = HashSet::new();
    for x in 0..MAPHIGHT{
        for y in 0..MAPWIDTH {
            let mapvar = perlin.get([x as f64 + 0.1, y as f64 + 0.1]);
            if mapvar < 0.2 {
                continue;
            }
            // THIS IS JUST A DEBUG MESSAGE DO NOT LEAVE IN
            println!("{}", mapvar);
            tiles.insert((x, y));
        }
    }

    for (x, y) in tiles.iter() {
        let (x, y) = grid_to_world(*x as f32, *y as f32);
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(1),
                transform: Transform::from_scale(Vec3::splat(SPRITESCALE as f32)).with_translation(vec3(*x, *y, 0.0)),
                ..default()
            },
    
        ));
    }

}

fn grid_to_world(x: f32, y: f32) -> (f32, f32){
    (
        x * MAPWIDTH as f32 * SPRITESCALE as f32,
        y * MAPHIGHT as f32 * SPRITESCALE as f32
    )
}
