use glam::Vec2;
use crate::block::{Block, BlockGroup};
use crate::common::GameArea;
use crate::constants::*;
use crate::monster::{Monster, MonsterClass, MonsterGroup};
use crate::random::RandomGenerator;
use crate::ship::Ship;


//decribes structure of each level
pub struct Level {
    monster_str: String,
    monster_laser_speed_lvl: i32,
    monster_laser_timer_lvl: i32,
    monster_speed_lvl: i32,
    ship_lives: i32,
    ship_laser_timer_lvl: i32,
    covers: i32,
    cover_width: i32,
    cover_height: i32,
    random_generator: RandomGenerator
}

impl Level {

    pub fn empty() -> Level {
        Level {
            monster_str: "".to_owned(),
            monster_laser_speed_lvl: 0,
            monster_laser_timer_lvl: 0,
            monster_speed_lvl: 0,
            ship_lives: 0,
            ship_laser_timer_lvl: 0,
            covers: 0,
            cover_width: 0,
            cover_height: 0,
            random_generator: RandomGenerator::new(0)
        }
    }

    pub fn create(monster_str: &str) -> Level {
        Level {
            monster_str: monster_str.to_owned(),
            monster_laser_speed_lvl: 0,
            monster_laser_timer_lvl: 0,
            monster_speed_lvl: 0,
            ship_lives: SHIP_INIT_LIVES,
            ship_laser_timer_lvl: 0,
            covers: 3,
            cover_width: 8,
            cover_height: 4,
            random_generator: RandomGenerator::new(0)
        }
    }

    pub fn set_monster_laser_speed_level(&mut self, value: i32) {
        self.monster_laser_speed_lvl = value;
    }

    pub fn set_monster_laser_timer_level(&mut self, value: i32) {
        self.monster_laser_timer_lvl = value;
    }

    pub fn set_monster_speed_level(&mut self, value: i32) {
        self.monster_speed_lvl = value;
    }

    pub fn set_ship_laser_timer_level(&mut self, value: i32) {
        self.ship_laser_timer_lvl = value;
    }

    pub fn set_ship_lives(&mut self, value: i32) {
        self.ship_lives = value;
    }

    //monster_str should be something like 'HHHHHHHHHHHFFFFFFFFFFFD H D H DH S B'
    // H - Heavy Fighter
    // F - Fighter
    // D - Defender with shield value
    // B - Bomber
    // S - Spawner
    // R - Random Monster
    // everything else is empty space. Columns are of constant width. Always MONSTERS_MAX_COLS
    pub fn build_monsters(&mut self, game_area: &GameArea) -> MonsterGroup {
        let mut monster_group = MonsterGroup::new(game_area);
        let monster_start_position = Vec2::new(game_area.left, game_area.top + 2.0 * MONSTER_HEIGHT);

        for i in 0..self.monster_str.len() {
            let m = self.monster_str.chars().nth(i).unwrap();
            let class = match m {
                'H' => Some(MonsterClass::HeavyFighter),
                'F' => Some(MonsterClass::Fighter),
                'D' => Some(MonsterClass::Defender(1)),
                'B' => Some(MonsterClass::Bomber),
                'S' => Some(MonsterClass::Spawner),
                'R' => Some(MonsterClass::random(&mut self.random_generator)),
                ' ' => None,
                _ => {
                    println!("Unknown monster class '{m}'");
                    None
                },
            };

            if let Some(class) = class {
                let col = i % MONSTERS_MAX_COLS;
                let row = i / MONSTERS_MAX_COLS;
                let p = Vec2::new(MONSTER_CELL_WIDTH * col as f32, MONSTER_CELL_HEIGHT * row as f32) + monster_start_position;
                let monster = Monster::new(p, class);
                monster_group.add_monster(monster);
            }
        }

        //set parameters for monster group
        monster_group.set_speed(MONSTER_INIT_SPEED + (self.monster_speed_lvl as f32 * MONSTER_SPEED_MOD));
        monster_group.set_laser_speed(MONSTER_INIT_LASER_SPEED + (self.monster_laser_speed_lvl as f32 * MONSTER_LASER_SPEED_MOD));
        monster_group.set_laser_timer(MONSTER_INIT_LASER_TIMER * MONSTER_LASER_TIMER_MOD.powi(self.monster_laser_timer_lvl));
        monster_group
    }

    pub fn build_ship(&self, game_area: &GameArea) -> Ship {
        let mut ship = Ship::new(game_area);
        ship.set_laser_timer(SHIP_INIT_LASER_TIMER * SHIP_LASER_TIMER_MOD.powi(self.ship_laser_timer_lvl));
        ship.set_lives(self.ship_lives);
        ship
    }

    pub fn build_covers(&self, game_area: &GameArea) -> BlockGroup {

        let mut block_group = BlockGroup::new();

        //divide game area into equal spaces for each cover
        if self.covers > 0 {
            let space_width = GAME_AREA_WIDTH / self.covers as f32;

            //for every space/cover
            for space_num in 0..self.covers {

                //get initial cover position. Cover is centered in its space
                let pos_x = game_area.left + space_num as f32 * space_width + (space_width - BLOCK_WIDTH * self.cover_width as f32) / 2.0;
                let p0 = Vec2::new(pos_x, game_area.bottom - 80.0);

                //generate every block in cover
                for r in 0..self.cover_height {
                    for w in 0..self.cover_width {
                        let mut p = p0;
                        p.x += BLOCK_WIDTH * w as f32;
                        p.y -= BLOCK_HEIGHT * r as f32;
                        block_group.add_block(Block::new(p));
                    }
                }
            }
        }
        block_group
    }
}
