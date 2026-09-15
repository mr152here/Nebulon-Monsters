use glam::Vec2;
use crate::audio_manager::{AudioManager, SoundType};
use crate::bomb::{Bomb, BombGroup};
use crate::common::GameArea;
use crate::constants::*;
use crate::laser::{Laser, LaserGroup};
use crate::random::RandomGenerator;


#[derive(Clone,Copy,PartialEq)]
pub enum MonsterClass {
    Fighter,
    HeavyFighter,
    Defender(i32),
    Bomber,
    Spawner
}

impl MonsterClass {

    pub fn random(random_generator: &mut RandomGenerator) -> MonsterClass {
        let s = "FHDBS";
        let idx = random_generator.next_number() as usize % s.len();
        match s.chars().nth(idx).unwrap() {
            'H' => MonsterClass::HeavyFighter,
            'F' => MonsterClass::Fighter,
            'D' => MonsterClass::Defender(1),
            'B' => MonsterClass::Bomber,
            'S' => MonsterClass::Spawner,
            _ => panic!("Undefined monster class")
        }
    }
}

#[derive(Clone,Copy,PartialEq)]
pub enum MonsterState {
    Alive,
    Hit(f32),
    Dead
}

pub struct Monster {
    position: Vec2,
    class: MonsterClass,
    state: MonsterState,
}

impl Monster {

    pub fn new(position: Vec2, class: MonsterClass) -> Monster {
        Monster {
            position,
            class,
            state: MonsterState::Alive,
        }
    }

    pub fn update(&mut self, speed: f32, dt: f32, descent: bool) {
        self.position.x += speed * dt;

        if descent {
            self.position.y += MONSTER_VERTICAL_STEP;
        }

        if let MonsterState::Hit(mut t) = self.state {
            t -= dt;
            if t <= 0.0 {
                if let MonsterClass::Defender(s) = self.class {
                    self.state = if s < 0 { MonsterState::Dead } else { MonsterState::Alive };
                } else {
                    self.state = MonsterState::Dead;
                }
            } else {
                self.state = MonsterState::Hit(t);
            }
        }
    }

    pub fn class(&self) -> MonsterClass {
        self.class
    }

    pub fn hit(&mut self) {
        self.state = MonsterState::Hit(MONSTER_HIT_TIMER);

        if let MonsterClass::Defender(s) = self.class {
            self.class = MonsterClass::Defender(s - 1);
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn score(&self) -> i32 {
        match self.class {
            MonsterClass::Fighter => SCORE_FIGHTER,
            MonsterClass::HeavyFighter => SCORE_HEAVY_FIGHTER,
            MonsterClass::Defender(_) => SCORE_DEFENDER,
            MonsterClass::Bomber => SCORE_BOMBER,
            MonsterClass::Spawner => SCORE_SPAWNER,
        }
    }

    pub fn state(&self) -> MonsterState {
        self.state
    }
}


pub struct MonsterGroup {
    monsters: Vec<Monster>,
    random_generator: RandomGenerator,
    speed: f32,
    laser_speed: f32,
    laser_timer: f32,
    laser_timer_counter: f32,
    game_area: GameArea,
}

impl MonsterGroup {

    pub fn new(game_area: &GameArea) -> MonsterGroup {
        MonsterGroup {
            monsters: Vec::<Monster>::new(),
            random_generator: RandomGenerator::new(0xAAAAAAAAAAAAAAAA),
            speed: 0.0,
            laser_speed: 0.0,
            laser_timer: 0.0,
            laser_timer_counter: 0.0,
            game_area: *game_area
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn set_laser_speed(&mut self, speed: f32) {
        self.laser_speed = speed;
    }

    pub fn set_laser_timer(&mut self, timer: f32) {
        self.laser_timer = timer;
    }

    pub fn add_monster(&mut self, monster: Monster) {
        self.monsters.push(monster);
    }

    pub fn update(&mut self, laser_group: &mut LaserGroup, bomb_group: &mut BombGroup, audio_manager: &AudioManager, dt: f32) {

        //if any monster is on the left or right edge of the game area
        let mut descent = false;

        for m in &self.monsters {
            if m.state == MonsterState::Alive {
                if m.position.x <= self.game_area.left {
                    let new_speed = self.speed.abs();
                    descent = self.speed != new_speed;
                    self.speed = new_speed;
                    break;

                } else if m.position.x + MONSTER_WIDTH >= self.game_area.right {
                    let new_speed = -self.speed.abs();
                    descent = self.speed != new_speed;
                    self.speed = new_speed;
                    break;
                }
            }
        }

        //update all monsters and get number of active monsters
        let mut alive = 0;
        self.monsters.iter_mut()
            .for_each(|m| {
                m.update(self.speed, dt, descent);
                if m.state == MonsterState::Alive {
                    alive += 1;
                }
            });

        //update laser timer
        self.laser_timer_counter -= dt;

        //if monsters can fire again, pick a random monster, fire and restart timer
        if self.laser_timer_counter <= 0.0 && alive > 0 {
            
            let rand_idx = self.random_generator.next_number() as usize % alive;
            let monster = self.monsters.iter_mut().filter(|m| m.state == MonsterState::Alive).nth(rand_idx).unwrap();
            let mut laser_position = monster.position;

            match monster.class {
                MonsterClass::HeavyFighter => {
                    laser_position.y += MONSTER_CELL_HEIGHT - LASER_HEIGHT;
                    laser_group.add(Laser::new(laser_position, self.laser_speed));
                    laser_position.x += MONSTER_WIDTH - LASER_WIDTH;
                    laser_group.add(Laser::new(laser_position, self.laser_speed));
                },
                MonsterClass::Fighter => {
                    laser_position += Vec2 { x: (MONSTER_WIDTH - LASER_WIDTH) / 2.0, y: MONSTER_HEIGHT - LASER_HEIGHT };
                    laser_group.add(Laser::new(laser_position, self.laser_speed));
                },
                MonsterClass::Defender(_) => {
                    laser_position += Vec2 { x: (MONSTER_WIDTH - LASER_WIDTH) / 2.0, y: MONSTER_HEIGHT - LASER_HEIGHT};
                    laser_group.add(Laser::new(laser_position, self.laser_speed));
                },
                //TODO: bomb speed?
                MonsterClass::Bomber => {
                    laser_position += Vec2 { x: (MONSTER_WIDTH - BOMB_WIDTH / 2.0), y: MONSTER_HEIGHT - BOMB_HEIGHT};
                    bomb_group.add(Bomb::new(laser_position, self.laser_speed));
                },
                MonsterClass::Spawner => {
                    //lets try to find a dead monster that is nearby the spawner
                    let spawner_position = monster.position;
                    let dead = self.monsters.iter()
                        .filter(|m| m.state == MonsterState::Dead)
                        .filter(|m| (m.position - spawner_position).length_squared() <= MONSTER_CELL_HEIGHT*MONSTER_CELL_HEIGHT + MONSTER_CELL_WIDTH*MONSTER_CELL_WIDTH + 10.0)
                        .count();

                    //if exists replace it with random monster
                    if dead > 0 {
                        let rand_idx = self.random_generator.next_number() as usize % dead;
                        let monster = self.monsters.iter_mut()
                            .filter(|m| m.state == MonsterState::Dead)
                            .filter(|m| (m.position - spawner_position).length_squared() <= MONSTER_CELL_HEIGHT*MONSTER_CELL_HEIGHT + MONSTER_CELL_WIDTH*MONSTER_CELL_WIDTH + 10.0)
                            .nth(rand_idx).unwrap();
                        monster.state = MonsterState::Alive;
                        monster.class = MonsterClass::random(&mut self.random_generator);

                    //if not fire the laser
                    } else {
                        laser_position += Vec2 { x: (MONSTER_WIDTH - LASER_WIDTH) / 2.0, y: MONSTER_HEIGHT - LASER_HEIGHT};
                        laser_group.add(Laser::new(laser_position, self.laser_speed));
                    }
                }
            }

            self.laser_timer_counter = self.laser_timer;
            audio_manager.play_sound(SoundType::MonsterFire);
        }
    }

    pub fn move_monsters_up(&mut self) {
        self.monsters.iter_mut()
            .for_each(|m| m.position.y -= MONSTER_VERTICAL_STEP * 3.0);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Monster> {
        self.monsters.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Monster> {
        self.monsters.iter_mut()
     }
}
