use glam::{Vec2, Vec3};
use gl::types::*;


pub const AUDIO_CHANNELS: i32 = 12;

//game area dimenstion are 800x800 pixel in the screen center
pub const GAME_AREA_WIDTH: f32 = 800.0;
pub const GAME_AREA_HEIGHT: f32 = 800.0;

//binding points for uniform buffers
pub const UNIFORM_BUFFER_BINDING_MATRICES: GLuint = 0;

//texture units
pub const TEXTURE_UNIT_SPRITE_ATLAS: GLuint = gl::TEXTURE0;
pub const TEXTURE_UNIT_SPRITE_ATLAS_UNIFORM_IDX: GLint = 0;
pub const TEXTURE_UNIT_BITMAP_FONT: GLuint = gl::TEXTURE1;
pub const TEXTURE_UNIT_BITMAP_FONT_UNIFORM_IDX: GLint = 1;

//sprites texture definitions
pub const TEXTURE_ATLAS_COLS: u32 = 4;
pub const TEXTURE_ATLAS_ROWS: u32 = 7;
pub const TEXTURE_ATLAS_H_STEP: f32 = 1.0/TEXTURE_ATLAS_COLS as f32;
pub const TEXTURE_ATLAS_V_STEP: f32 = 1.0/TEXTURE_ATLAS_ROWS as f32;
pub const TEXTURE_ATLAS_FIGHTER_OFFSET: Vec2 = Vec2::new(0.0, 0.0);
pub const TEXTURE_ATLAS_HEAVY_FIGHTER_OFFSET: Vec2 = Vec2::new(0.0, TEXTURE_ATLAS_V_STEP);
pub const TEXTURE_ATLAS_DEFENDER_OFFSET: Vec2 = Vec2::new(0.0, TEXTURE_ATLAS_V_STEP * 2.0);
pub const TEXTURE_ATLAS_BOMBER_OFFSET: Vec2 = Vec2::new(0.0, TEXTURE_ATLAS_V_STEP * 3.0);
pub const TEXTURE_ATLAS_SPAWNER_OFFSET: Vec2 = Vec2::new(0.0, TEXTURE_ATLAS_V_STEP * 4.0);

//score max and count
pub const SCORE_MAX: i32 = 99999;
pub const TOP_SCORE_COUNT: usize = 20;

//number of seconds for each frame
pub const ANIMATION_FRAME_TIMER: f32 = 0.6;
pub const MONSTERS_ANIMATION_FRAME_COUNT: u32 = TEXTURE_ATLAS_COLS;
pub const UFO_ANIMATION_FRAME_COUNT: u32 = TEXTURE_ATLAS_COLS / 2;

//colors for menu, background and font
pub const BACKGROUND_COLOR: Vec3 = Vec3::new(0.05, 0.05, 0.05);
pub const FONT_COLOR: Vec3 = Vec3::new(1.0, 0.8, 0.1);
pub const MENU_COLOR: Vec3 = Vec3::new(1.0, 0.8, 0.1);
pub const MENU_SELECTED_COLOR: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub const SCORE_COLOR: Vec3 = Vec3::new(0.8, 0.8, 0.8);
pub const VERSION_COLOR: Vec3 = Vec3::new(0.25, 0.25, 0.25);

//ship
pub const SHIP_INIT_LIVES: i32 = 4;
pub const SHIP_WIDTH: f32 = 1.5 * MONSTER_WIDTH;
pub const SHIP_HEIGHT: f32 = 0.6 * MONSTER_HEIGHT;
pub const SHIP_INIT_SPEED: f32 = GAME_AREA_WIDTH / 3.5;
pub const SHIP_INIT_LASER_TIMER: f32 = 1.3;
pub const SHIP_LASER_TIMER_MOD: f32 = 0.959;
pub const SHIP_INIT_LASER_SPEED: f32 = -700.0;
pub const SHIP_HIT_TIMER: f32 = MONSTER_HIT_TIMER;
pub const SHIP_COLOR: Vec3 = Vec3::new(230.0/255.0, 200.0/255.0, 10.0/255.0);
pub const SHIP_COLOR_HIT: Vec3 = Vec3::new(1.0, 1.0, 1.0);

//laser, bomb and block
pub const LASER_WIDTH: f32 = 3.0;
pub const LASER_HEIGHT: f32 = 14.0;
pub const LASER_COLOR: Vec3 = Vec3::new(0.5, 0.8, 0.0);
pub const BOMB_TIMER: f32 = 2.5;
pub const BOMB_WIDTH: f32 = BLOCK_WIDTH;
pub const BOMB_HEIGHT: f32 = BLOCK_HEIGHT;
pub const BOMB_COLOR: Vec3 = MONSTER_COLOR_BOMBER;
pub const BLOCK_WIDTH: f32 = 10.0;
pub const BLOCK_HEIGHT: f32 = 10.0;
pub const BLOCK_COLOR: Vec3 = Vec3::new(125.0/255.0, 125.0/255.0, 125.0/255.0);

//stars
pub const STAR_MAX_COUNT: i32 = 2000;
pub const STAR_SIZE_MIN: u64 = 2;
pub const STAR_SIZE_MAX: u64 = 4;
pub const STAR_LIFESPAN_MIN: u64 = 3000;
pub const STAR_LIFEPAN_MAX: u64 = 10000;
pub const STAR_BASE_COLOR: Vec3 = Vec3::new(0.4, 0.4, 0.4);

//monsters
pub const MONSTER_WIDTH: f32 = 40.0;
pub const MONSTER_HEIGHT: f32 = 40.0;
pub const MONSTERS_MAX_COLS: usize = 11;
pub const MONSTER_INIT_SPEED: f32 = GAME_AREA_WIDTH / 16.0;
pub const MONSTER_SPEED_MOD: f32 = MONSTER_INIT_SPEED * 0.045;
pub const MONSTER_VERTICAL_STEP: f32 = MONSTER_HEIGHT / 2.0;
pub const MONSTER_INIT_LASER_SPEED: f32 = 210.0;
pub const MONSTER_LASER_SPEED_MOD: f32 = MONSTER_INIT_LASER_SPEED * 0.04;
pub const MONSTER_INIT_LASER_TIMER: f32 = 1.6;
pub const MONSTER_LASER_TIMER_MOD: f32 = 0.962;
pub const MONSTER_HIT_TIMER: f32 = 0.1;
pub const MONSTER_COLOR_FIGHTER: Vec3 = Vec3::new(248.0/255.0, 208.0/255.0, 1.0/255.0);
pub const MONSTER_COLOR_HEAVY_FIGHTER: Vec3 = Vec3::new(209.0/255.0, 29.0/255.0, 32.0/255.0);
pub const MONSTER_COLOR_DEFENDER: Vec3 = Vec3::new(50.0/255.0, 100.0/255.0, 240.0/255.0);
pub const MONSTER_COLOR_BOMBER: Vec3 = Vec3::new(149.0/255.0, 53.0/255.0, 236.0/255.0);
pub const MONSTER_COLOR_SPAWNER: Vec3 = Vec3::new(60.0/255.0, 170.0/255.0, 20.0/255.0);
pub const MONSTER_COLOR_HIT: Vec3 = Vec3::new(1.0, 1.0, 1.0);

//grid / cell size for monsters
pub const MONSTER_CELL_WIDTH: f32 = MONSTER_WIDTH + 18.0;
pub const MONSTER_CELL_HEIGHT: f32 = MONSTER_HEIGHT + 20.0;

//ufo
pub const UFO_INIT_SPEED_P: f32 = GAME_AREA_WIDTH / 4.0;
pub const UFO_INIT_SPEED_N: f32 = -UFO_INIT_SPEED_P;
pub const UFO_WIDTH: f32 = 2.0 * MONSTER_WIDTH;
pub const UFO_HEIGHT: f32 = MONSTER_HEIGHT;
pub const UFO_HIT_TIMER: f32 = MONSTER_HIT_TIMER;
pub const UFO_MINIMAL_SPAWN_INTERVAL: u64 = 10;
pub const UFO_RANDOM_SPAWN_INTERVAL: u64 = 10;
pub const UFO_COLOR: Vec3 = Vec3::new(1.0, 170.0/255.0, 0.0);
pub const UFO_COLOR_HIT: Vec3 = Vec3::new(1.0, 1.0, 1.0);

//score values per hit
pub const SCORE_FIGHTER: i32 = 2;
pub const SCORE_HEAVY_FIGHTER: i32 = 5;
pub const SCORE_DEFENDER: i32 = 4;
pub const SCORE_BOMBER: i32 = 8;
pub const SCORE_SPAWNER: i32 = 8;
pub const SCORE_BOMB: i32 = 10;
pub const SCORE_LIVE: i32 = 20;
pub const SCORE_UFO: i32 = 200;
