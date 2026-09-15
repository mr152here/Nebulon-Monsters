use glam::Vec2;
use crate::audio_manager::{AudioManager, SoundType};
use crate::common::GameArea;
use crate::constants::*;
use crate::laser::{Laser, LaserGroup};


#[derive(Clone,Copy,PartialEq)]
pub enum ShipState {
    Alive,
    Hit(f32),
    Dead
}

//Ship struct is practicaly a player.
pub struct Ship {
    position: Vec2,
    state: ShipState,
    lives: i32,
    hit: bool,
    laser_timer: f32,
    laser_timer_counter: f32,
    game_area: GameArea,
}

impl Ship {

    pub fn new(game_area: &GameArea) -> Ship {

        let position = Vec2 {
            x: game_area.left + (GAME_AREA_WIDTH - SHIP_WIDTH) / 2.0,
            y: game_area.bottom - SHIP_HEIGHT
        };

        Ship {
            position,
            state: ShipState::Alive,
            lives: SHIP_INIT_LIVES,
            hit: false,
            laser_timer: SHIP_INIT_LASER_TIMER,
            laser_timer_counter: 0.0,
            game_area: *game_area,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn move_right(&mut self, dt: f32) {
        self.position.x = (self.position.x + SHIP_INIT_SPEED * dt).min(self.game_area.right - SHIP_WIDTH);
    }

    pub fn move_left(&mut self, dt: f32) {
        self.position.x = (self.position.x - SHIP_INIT_SPEED * dt).max(self.game_area.left);
    }

    pub fn update(&mut self, dt: f32) {
        self.hit = false;

        if self.state == ShipState::Dead {
            return;
        }

        if let ShipState::Hit(mut t) = self.state {
            t -= dt;
            if t <= 0.0 {
                self.state = if self.lives >= 0 { ShipState::Alive } else { ShipState::Dead };
            } else {
                self.state = ShipState::Hit(t);
            }
        }

        if self.laser_timer_counter > 0.0 {
            self.laser_timer_counter -= dt;
        }
    }

    //when ship was hit by something
    pub fn hit(&mut self) {
        self.lives -= 1;
        self.state = ShipState::Hit(SHIP_HIT_TIMER);
        self.hit = true;
    }

    //return whether ship was hit after the last update
    pub fn was_hit(&self) -> bool {
        self.hit
    }

    pub fn lives(&self) -> i32 {
        self.lives
    }

    pub fn set_lives(&mut self, lives: i32) {
        self.lives = lives;
    }

    pub fn set_laser_timer(&mut self, timer: f32) {
        self.laser_timer = timer;
    }

    pub fn state(&self) -> ShipState {
        self.state
    }

    //fire ship's laser
    pub fn fire(&mut self, laser_manager: &mut LaserGroup, audio_manager: &AudioManager) {
        if self.state != ShipState::Dead && self.laser_timer_counter <= 0.0 {
            self.laser_timer_counter = self.laser_timer;

            let laser_position = Vec2::new(self.position.x + (SHIP_WIDTH - LASER_WIDTH) / 2.0, self.position.y);
            laser_manager.add(Laser::new(laser_position, SHIP_INIT_LASER_SPEED));
            audio_manager.play_sound(SoundType::ShipFire);
        }
    }
}
