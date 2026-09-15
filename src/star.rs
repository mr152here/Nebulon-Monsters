use glam::{Vec2, Vec3};

use crate::common::GameArea;
use crate::constants::*;
use crate::random::RandomGenerator;


pub struct Star {
    position: Vec2,
    size: Vec2,
    color: Vec3,
    delta_color: Vec3,
    lifespan: f32,
    timer: f32
}

impl Star {
    
    pub fn new(position: &Vec2, size: &Vec2, color: &Vec3, lifespan: f32) -> Star {
        let delta_color = 2.0 * (color - BACKGROUND_COLOR) / lifespan;

        Star {
            position: *position,
            size: *size,
            delta_color,
            color: BACKGROUND_COLOR,
            lifespan,
            timer: lifespan
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.timer -= delta_time;

        let color_timer = self.lifespan - self.timer;
        self.color += if color_timer <= self.lifespan / 2.0 { self.delta_color } else {-self.delta_color } * delta_time;
    }

    fn reset_timer(&mut self) {
        self.timer = self.lifespan;
    }

    pub fn color(&self) -> Vec3 {
        self.color
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}


pub struct StarGroup {
    stars: Vec<Star>,
    game_area: GameArea,
    random_generator: RandomGenerator,
}

impl StarGroup {

    pub fn new(game_area: &GameArea, mount: i32) -> StarGroup {
        let mut random_generator = RandomGenerator::new(0xBABE1234FFFFFFFF);
        let mut stars = Vec::<Star>::with_capacity(mount as usize);
        let cb = ((1.0 - STAR_BASE_COLOR.z) * 255.0) as u64;

        for _ in 0..mount {
            let pos_x = (random_generator.next_number() % GAME_AREA_WIDTH as u64) as f32 + game_area.left;
            let pos_y = (random_generator.next_number() % GAME_AREA_HEIGHT as u64) as f32 + game_area.top;
            let position = Vec2::new(pos_x, pos_y);

            let size = (STAR_SIZE_MIN + random_generator.next_number() % (STAR_SIZE_MAX - STAR_SIZE_MIN + 1)) as f32;
            let size = Vec2::new(size, size);

            let lifespan = (STAR_LIFESPAN_MIN + (random_generator.next_number() % (STAR_LIFEPAN_MAX - STAR_LIFESPAN_MIN))) as f32 / 1000.0;

            let mut color = STAR_BASE_COLOR;
            color.z += (random_generator.next_number() % cb) as f32 / 255.0;

            let p = Star::new(&position, &size, &color, lifespan);
            stars.push(p);
        }

        StarGroup {
            stars: stars,
            game_area: *game_area,
            random_generator,
        }
    }

    pub fn update(&mut self, delta_time: f32) {

        //update all stars and respawn dead on new random position
        self.stars.iter_mut()
            .for_each(|s| {
                s.update(delta_time);

                if s.timer <= 0.0 {
                    let rx = self.game_area.left + (self.random_generator.next_number() % GAME_AREA_WIDTH as u64) as f32;
                    let ry = self.game_area.top + (self.random_generator.next_number() % GAME_AREA_HEIGHT as u64) as f32;
                    s.position.x = rx;
                    s.position.y = ry;
                    s.reset_timer();
                }
            });
    }

    pub fn count(&self) -> usize {
        self.stars.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Star> {
        self.stars.iter()
    }
}
