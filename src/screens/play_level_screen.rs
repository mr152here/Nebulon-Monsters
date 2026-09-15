use crate::audio_manager::{AudioManager, SoundType};
use crate::block::BlockGroup;
use crate::bomb::BombGroup;
use crate::common::{AABB, DisplayInfo};
use crate::constants::*;
use crate::game::{GameState, PlayerCommand};
use crate::gui::{Gui, GuiRenderer};
use crate::laser::LaserGroup;
use crate::level::Level;
use crate::monster::{MonsterGroup, MonsterState};
use crate::star::StarGroup;
use crate::renderers::{MonsterRenderer, ShipRenderer, SimpleRenderer, StarRenderer, UfoRenderer};
use crate::shader_manager::ShaderManager;
use crate::ship::Ship;
use crate::ufo::{Ufo, UfoState};


pub struct PlayLevelScreen {
    ship: Ship,
    ship_renderer: ShipRenderer,
    monster_group: MonsterGroup,
    monster_renderer : MonsterRenderer,
    laser_group: LaserGroup,
    block_group: BlockGroup,
    bomb_group: BombGroup,
    star_group: StarGroup,
    star_renderer: StarRenderer,
    ufo: Ufo,
    ufo_renderer: UfoRenderer,
    gui: Gui,
    gui_renderer:GuiRenderer,
    display_info: DisplayInfo,
    score: i32,
    pause_on_hit: f32,
    simple_renderer: SimpleRenderer
}

impl PlayLevelScreen {

    pub fn new(level: &mut Level, display_info: &DisplayInfo, pause_on_hit: f32, score: i32, stars: i32, shader_manager: &ShaderManager) -> PlayLevelScreen {

        //build all level objects
        let game_area = &display_info.game_area;
        let ship = level.build_ship(game_area);
        let monster_group = level.build_monsters(game_area);
        let block_group = level.build_covers(game_area);
        let star_group = StarGroup::new(game_area, stars);

        PlayLevelScreen {
            ship,
            ship_renderer: ShipRenderer::new(shader_manager),
            monster_group,
            monster_renderer: MonsterRenderer::new(shader_manager),
            laser_group: LaserGroup::new(game_area),
            block_group,
            star_group,
            star_renderer: StarRenderer::new(shader_manager),
            bomb_group: BombGroup::new(game_area),
            ufo: Ufo::new(game_area),
            ufo_renderer: UfoRenderer::new(shader_manager),
            gui: Gui::new(game_area),
            gui_renderer: GuiRenderer::new(game_area, shader_manager),
            display_info: *display_info,
            score,
            pause_on_hit,
            simple_renderer: SimpleRenderer::new(shader_manager, game_area),
        }
    }

    pub fn reset_score(&mut self) {
        self.score = 0;
    }

    fn process_collisions(&mut self, audio_manager: &AudioManager) {

        //TODO: this is can be optimized. Is not necessary to recreate all those AABBs. They may be stored in "xxx_group/manager" and update them when needed
        //lasers with monsters, covers, bombs and ship
        let s_position = self.ship.position();
        let s_aabb = AABB::new(s_position.y, s_position.y + SHIP_HEIGHT, s_position.x, s_position.x + SHIP_WIDTH);

        //for every laser bolt, check if its AABB collide with something
        'lasers: for l_ref in self.laser_group.iter_mut() {
            if let Some(l) = l_ref {
                let l_position = l.position();
                let l_aabb = AABB::new(l_position.y, l_position.y + LASER_HEIGHT, l_position.x, l_position.x + LASER_WIDTH);

                //players shoots (up going lasers)
                if l.speed().is_sign_negative() {

                    //collision with monsters
                    for m in self.monster_group.iter_mut() {
                        if m.state() == MonsterState::Alive {
                            let m_position = m.position();
                            let m_aabb = AABB::new(m_position.y, m_position.y + MONSTER_HEIGHT, m_position.x, m_position.x + MONSTER_WIDTH);

                            //if hit, destroy laser and monster
                            if l_aabb.collide(&m_aabb) {
                                *l_ref = None;
                                m.hit();
                                self.score = i32::min(self.score + m.score(), SCORE_MAX);
                                audio_manager.play_sound(SoundType::MonsterHit);
                                continue 'lasers;
                            }
                        }
                    }

                    //collision with blocks
                    for b_ref in self.block_group.iter_mut() {
                        if let Some(b) = b_ref {
                            let b_position = b.position();
                            let b_aabb = AABB::new(b_position.y, b_position.y + BLOCK_HEIGHT, b_position.x, b_position.x + BLOCK_WIDTH);

                            if l_aabb.collide(&b_aabb) {
                                *l_ref = None;
                                *b_ref = None;
                                continue 'lasers;
                            }
                        }
                    }

                    //collision with bombs
                    for b_ref in self.bomb_group.iter_mut() {
                        if let Some(b) = b_ref {
                            let b_position = b.position();
                            let b_aabb = AABB::new(b_position.y, b_position.y + BOMB_HEIGHT, b_position.x, b_position.x + BOMB_WIDTH);

                            if l_aabb.collide(&b_aabb) {
                                *l_ref = None;
                                *b_ref = None;
                                self.score = i32::min(self.score + SCORE_BOMB, SCORE_MAX);
                                audio_manager.play_sound(SoundType::BombHit);
                                continue 'lasers;
                            }
                        }
                    }

                    //collision with ufo
                    if self.ufo.state() == UfoState::Active {
                        let u_position = self.ufo.position();
                        let u_aabb = AABB::new(u_position.y, u_position.y + UFO_HEIGHT, u_position.x, u_position.x + UFO_WIDTH);

                        if l_aabb.collide(&u_aabb) {
                            *l_ref = None;
                            self.ufo.hit();
                            self.score = i32::min(self.score + SCORE_UFO, SCORE_MAX);
                            audio_manager.play_sound(SoundType::UfoHit);
                            // continue 'lasers;
                        }
                    }

                //monsters shoots collisions with ship
                } else if l_aabb.collide(&s_aabb) {
                    *l_ref = None;
                    self.ship.hit();
                    audio_manager.play_sound(SoundType::ShipHit);

                //monsters lasers with blocks
                } else {
                    for b_ref in self.block_group.iter_mut() {
                        if let Some(b) = b_ref {
                            let b_position = b.position();
                            let b_aabb = AABB::new(b_position.y, b_position.y + BLOCK_HEIGHT, b_position.x, b_position.x + BLOCK_WIDTH);

                            if l_aabb.collide(&b_aabb) {
                                *l_ref = None;
                                *b_ref = None;
                                break;
                            }
                        }
                    }
                }
            }
        }

        //monsters collisions with blocks, ship, bottom of the game area
        for m in self.monster_group.iter() {
            if m.state() == MonsterState::Alive {
                let m_position = m.position();
                let m_aabb = AABB::new(m_position.y, m_position.y + MONSTER_HEIGHT, m_position.x, m_position.x + MONSTER_WIDTH);

                for b_ref in self.block_group.iter_mut() {
                    if let Some(b) = b_ref {
                        let b_position = b.position();
                        let b_aabb = AABB::new(b_position.y, b_position.y + BLOCK_HEIGHT, b_position.x, b_position.x + BLOCK_WIDTH);

                        //destroy all block that collide with monster
                        if m_aabb.collide(&b_aabb) {
                            *b_ref = None;
                        }
                    }
                }

                //collision with the ship and with the bottom of the play area, Set live to -1 as a game over.
                if m_aabb.collide(&s_aabb) || m_position.y + MONSTER_HEIGHT >= self.display_info.game_area.bottom {
                    self.ship.set_lives(-1);
                    break;
                }
            }
        }

        //bombs
        'bombs: for b_ref in self.bomb_group.iter_mut() {
            if let Some(b) = b_ref {
                let b_position = b.position();
                let b_aabb = AABB::new(b_position.y, b_position.y + BOMB_HEIGHT, b_position.x, b_position.x + BOMB_WIDTH);

                //bomb collision with ship
                if b_aabb.collide(&s_aabb) {
                    *b_ref = None;
                    self.ship.hit();
                    audio_manager.play_sound(SoundType::ShipHit);
                    continue 'bombs;
                }

                //bomb collision with blocks
                for bl_ref in self.block_group.iter_mut() {
                    if let Some(bl) = bl_ref {
                        let bl_position = bl.position();
                        let bl_aabb = AABB::new(bl_position.y, bl_position.y + BLOCK_HEIGHT, bl_position.x, bl_position.x + BLOCK_WIDTH);

                        //destroy all blocks that are hit
                        if b_aabb.collide(&bl_aabb) {
                            *b_ref = None;
                            *bl_ref = None;
                        }
                    }
                }
            }
        }
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn set_pause_on_hit(&mut self, pause: f32) {
        self.pause_on_hit = pause
    }

    pub fn update(&mut self, commands: &[PlayerCommand], single_command: &Option<PlayerCommand>, level_number: i32, audio_manager: &AudioManager, delta_time: f32) -> GameState {

        //process single command
        if let Some(command) = single_command {
            match command {
                PlayerCommand::Escape => return GameState::PauseMenuScreen,
                _ => (),
            }
        }

        //process player commands
        for command in commands.iter() {
            match command {
                PlayerCommand::Left => self.ship.move_left(delta_time),
                PlayerCommand::Right => self.ship.move_right(delta_time),
                PlayerCommand::Fire => self.ship.fire(&mut self.laser_group, audio_manager),
                _ => (),
            }
        }

        //update ship
        self.ship.update(delta_time);

        //update monsters
        self.monster_group.update(&mut self.laser_group, &mut self.bomb_group, audio_manager, delta_time);

        //update UFO
        self.ufo.update(delta_time);

        //update lasers, bombs and particles
        self.laser_group.update(delta_time);
        self.bomb_group.update(delta_time);
        self.star_group.update(delta_time);

        //check for collisions
        self.process_collisions(audio_manager);

        //add player a +1 live and move monsters up if ufo was hit
        if self.ufo.was_hit() {
            self.ship.set_lives(self.ship.lives() + 1);
            self.monster_group.move_monsters_up();
        }

        //update GUI
        self.gui.update(level_number, self.score, self.ship.lives());

        //check if player wins the game
        if self.score >= SCORE_MAX {
            return GameState::WinGame;
        }

        //when ship was hit, pause game for a while
        if self.ship.was_hit() {
            return GameState::HitPause(self.pause_on_hit);
        }

        //if player is out of lives, it is game over
        if self.ship.lives() < 0 {
            return GameState::GameOver;

        //TODO: this is unnecessary to iterate it every time. This info can be stored during collisions.
        //when all monsters are destroyed, proceed to the next level
        } else if self.monster_group.iter().filter(|m| m.state() == MonsterState::Alive).count() == 0 {
            self.score = i32::min(self.score + self.ship.lives() * SCORE_LIVE, SCORE_MAX);
            return GameState::NextLevel;
        }
        GameState::PlayLevel
    }

    pub fn render(&mut self, shader_manager: &ShaderManager, delta_time: f32) {

        //render stars
        self.star_renderer.render(shader_manager, &self.star_group);

        //render ship
        self.ship_renderer.render(shader_manager, &self.ship);

        //render UFO
        self.ufo_renderer.render(shader_manager, &self.ufo, delta_time);

        //render lasers, bomb, blocks and ufo covers
        self.simple_renderer.render(shader_manager, &self.laser_group, &self.bomb_group, &self.block_group);

        //render monsters
        self.monster_renderer.render(shader_manager, &self.monster_group, delta_time);

        //render UI
        self.gui_renderer.render(shader_manager, &self.gui);
    }
}
