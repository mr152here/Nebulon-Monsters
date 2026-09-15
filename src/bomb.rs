use glam::Vec2;
use crate::common::GameArea;
use crate::constants::*;


//by default bomb is in inactive state. Activates when hit the ground
#[derive(Clone,Copy)]
pub enum BombState {
    Inactive,
    Active(f32),
    Expired
}

//bomb is a shot from bomber monster. Drops down and when it hits ground, it will stay there for a BOMB_TIMER seconds.
pub struct Bomb {
    position: Vec2,
    state: BombState,
    speed: f32
}

impl Bomb {

    pub fn new(position: Vec2, speed: f32) -> Bomb {
        Bomb {
            position,
            state: BombState::Inactive,
            speed
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.state {
            BombState::Active(t) => {
                let new_t = t - dt;
                self.state = if new_t <= 0.0 { BombState::Expired } else { BombState::Active(new_t) };
            },
            BombState::Inactive => {
                self.position.y += self.speed * dt;
            },
            _ => ()
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn state(&self) -> BombState {
        self.state
    }
}


pub struct BombGroup {
    bombs: Vec<Option<Bomb>>,
    game_area: GameArea
}

impl BombGroup {

    pub fn new(game_area: &GameArea) -> BombGroup {
        BombGroup {
            bombs: Vec::new(),
            game_area: *game_area
        }
    }

    //append a new bomb or replace "expired"
    pub fn add(&mut self, bomb: Bomb) {

        if let Some(r) = self.bombs.iter_mut().find(|b| b.is_none()) {
            *r = Some(bomb);
        } else {
            self.bombs.push(Some(bomb));
        }
    }

    //update all bombs
    pub fn update(&mut self, dt: f32) {

        self.bombs.iter_mut()
            .filter(|b| b.is_some())
            .for_each(|b| {
                let bb = b.as_mut().unwrap();
                bb.update(dt);
                let position = bb.position().y;

                //activate timer when bomb hit the ground
                match bb.state() {
                    BombState::Inactive => {
                        if position + BOMB_HEIGHT >= self.game_area.bottom {
                            bb.position.y = self.game_area.bottom - BOMB_HEIGHT;
                            bb.state = BombState::Active(BOMB_TIMER);
                        }
                    },
                    BombState::Expired => {
                        *b = None;
                    },
                    _ => ()
                }
            });
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Bomb>> {
        self.bombs.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Option<Bomb>> {
        self.bombs.iter_mut()
    }
}
