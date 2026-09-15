use glam::Vec2;
use crate::common::GameArea;
use crate::constants::*;


pub struct Laser {
    position: Vec2,
    speed: f32
}

impl Laser {

    pub fn new(position: Vec2, speed: f32) -> Laser {
        Laser {
            position,
            speed
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position.y += self.speed * dt;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}

//groups all lasers shots into single object. For rendering and updating.
pub struct LaserGroup {
    lasers: Vec<Option<Laser>>,
    game_area: GameArea
}

impl LaserGroup {

    pub fn new(game_area: &GameArea) -> LaserGroup {
        LaserGroup {
            lasers: Vec::new(),
            game_area: *game_area
        }
    }

    //append new laser shot or replace "expired" one
    pub fn add(&mut self, laser: Laser) {

        if let Some(r) = self.lasers.iter_mut().find(|l| l.is_none()) {
            *r = Some(laser);
        } else {
            self.lasers.push(Some(laser));
        }
    }

    //update all lasers and remove that are out of game area
    pub fn update(&mut self, dt: f32) {

        self.lasers.iter_mut()
            .filter(|l| l.is_some())
            .for_each(|l| {
                let ll = l.as_mut().unwrap();
                ll.update(dt);
                let speed = ll.speed();
                let position = ll.position().y;

                //when laser is out of play area, remove it
                if speed.is_sign_negative() && position < self.game_area.top || speed.is_sign_positive() && (position + LASER_HEIGHT) > self.game_area.bottom {
                    *l = None;
                }
            });
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Laser>> {
        self.lasers.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Option<Laser>> {
        self.lasers.iter_mut()
    }
}
