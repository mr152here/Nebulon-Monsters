use glam::Vec2;
use crate::common::GameArea;
use crate::constants::*;
use crate::random::RandomGenerator;


#[derive(Clone,Copy,PartialEq)]
pub enum UfoState {
    Active,
    Inactive(f32),
    Hit(f32),
    Dead
}

//UFO flyes at the top of the game area. Spawn at random intervals and at random sides. When destroyed doesn't spawn any more.
pub struct Ufo {
    position: Vec2,
    speed: f32,
    hit: bool,
    state: UfoState,
    game_area: GameArea,
    rng: RandomGenerator,
}

impl Ufo {

    pub fn new(game_area: &GameArea) -> Ufo {
        let mut rng = RandomGenerator::new(0);
        let timer = (UFO_MINIMAL_SPAWN_INTERVAL + rng.next_number() % UFO_RANDOM_SPAWN_INTERVAL) as f32;

        Ufo {
            position: Vec2::ZERO,
            speed: 0.0,
            hit: false,
            state: UfoState::Inactive(timer),
            game_area: *game_area,
            rng,
        }
    }

    pub fn hit(&mut self) {
        if self.state == UfoState::Active {
            self.state = UfoState::Hit(UFO_HIT_TIMER);
            self.hit = true;
        }
    }

    pub fn was_hit(&self) -> bool {
        self.hit
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn state(&self) -> UfoState {
        self.state
    }

    pub fn update(&mut self, dt: f32) {
        self.hit = false;
        
        match self.state {
            UfoState::Inactive(t) => {
                self.state = if t - dt <= 0.0 {
                    if self.rng.next_number() % 2 == 1 {
                        self.position = Vec2::new(self.game_area.left - UFO_WIDTH, self.game_area.top + 15.0);
                        self.speed = UFO_INIT_SPEED_P;
                    } else {
                        self.position = Vec2::new(self.game_area.right, self.game_area.top + 15.0);
                        self.speed = UFO_INIT_SPEED_N;
                    }
                    UfoState::Active
                } else {
                    UfoState::Inactive(t - dt)
                };
            },
            UfoState::Active => {
                self.position.x += self.speed * dt;
                if self.position.x < self.game_area.left - UFO_WIDTH || self.position.x > self.game_area.right {
                    self.state = UfoState::Inactive((UFO_MINIMAL_SPAWN_INTERVAL + self.rng.next_number() % UFO_RANDOM_SPAWN_INTERVAL) as f32);
                }
            },
            UfoState::Hit(t) => {
                self.state = if t - dt <= 0.0 { UfoState::Dead } else { UfoState::Hit(t - dt) };
            },
            _ => (),
        }
    }
}
