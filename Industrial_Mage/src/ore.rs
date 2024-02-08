// This should hold all Logic to do with Ore and Maybe all functions associated with those systems
// Also World Generation!!!
use bevy::{math::vec3, prelude::*, render::texture, transform::commands, utils::hashbrown::HashSet};
use noise::Perlin;
use noise::NoiseFn;
use rand::Rng;


pub struct OreLogicPlugin;

impl Plugin for OreLogicPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, genesis);
    }
}

// Below was built using the 2d Sprite Sheets demo from assets
// Below is the Map Details
const MAPHIGHT: usize = 80;
const MAPWIDTH: usize = 80;

// Sprite Details 
const TILE_HEIGHT: usize = 64;
const TILE_WIDTH: usize = 64;
const SPRITESHEET_HEIGHT: usize = 128 / TILE_HEIGHT;
const SPRITESHEET_WIDTH: usize = 640 / TILE_WIDTH;
const TEXTURE_PATH: &str = "terra/terrain.png";
const SPRITESCALE: usize = 4;
// Perlin Noise scale
const NOISE_SCALE: f64 = 10.5;

fn genesis(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // BELOW IS USED TO HANDLE THE LOADING OF MAP ASSETS
    let texture_handle = asset_server.load(TEXTURE_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32), SPRITESHEET_WIDTH, SPRITESHEET_HEIGHT, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Below is the actual Generation of some of the map assets!
    let mut rngseed = rand::thread_rng();
    let perlin: Perlin = Perlin::new(rngseed.gen());
    

    let mut tiles = HashSet::new();
    for x in 0..MAPHIGHT{
        for y in 0..MAPWIDTH {
            let mapvar = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
            if mapvar < 0.2 {
                // Make this Value The hard rock!
                continue;
            } else {
                
            }
            // THIS IS JUST A DEBUG MESSAGE DO NOT LEAVE IN
            println!("{}", mapvar);
            tiles.insert((x as i32, y as i32));
        }
    }

    for (x, y) in tiles.iter() {
        let tile = get_tile((*x, *y), &tiles);
        let (x, y) = grid_to_world(*x as f32, *y as f32);
        
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(tile),
                transform: Transform::from_scale(Vec3::splat(SPRITESCALE as f32)).with_translation(vec3(x, y, 0.0)),
                ..default()
            },
    
        ));
    }

}

fn get_tile((x, y): (i32, i32), occupied: &HashSet<(i32, i32)>) -> usize {
    let (x, y) = (x as i32, y as i32);
    let neighbor_options = [(-1, 0), (1, 0),(0,-1),(0,1) ];
    let mut neightbor = [1,1,1,1];
    for (idx, (i, j)) in neighbor_options.iter().enumerate() {
        if occupied.contains(&(x + i, y + j)) {
            continue;
        }
        neightbor[idx] = 0;
    }
    let tile = match neightbor {
        [1,0,0,1] => 3,
        [0,1,0,1] => 4,
        [1,0,1,0] => 2, 
        [0,1,1,0] => 1, //correct

        _ => 0
    };
    tile

    
}








fn grid_to_world(x: f32, y: f32) -> (f32, f32){
    (
        x * TILE_WIDTH as f32 * SPRITESCALE as f32,
        y * TILE_HEIGHT as f32 * SPRITESCALE as f32
    )
}
