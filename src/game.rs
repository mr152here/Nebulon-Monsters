use crate::audio_manager::{AudioManager, SoundType};
use crate::common::DisplayInfo;
use crate::configuration::Configuration;
use crate::constants::*;
use crate::level::Level;
use crate::screens::game_over_screen::GameOverScreen;
use crate::screens::main_menu_screen::{MainMenuScreen, PauseMenuScreen, SettingsMenuScreen};
use crate::screens::play_level_screen::PlayLevelScreen;
use crate::screens::top_score_screen::TopScoreScreen;
use crate::shader_manager::ShaderManager;

//TODO: move to texture manager
use image::{self, EncodableLayout};
use crate::ogl::texture::Texture2D;

#[derive(Clone,Copy,PartialEq)]
pub enum GameState {
    GameOverScreen,
    MainMenuScreen,
    PauseMenuScreen,
    SettingsMenuScreen,
    TopScoreScreen,
    WinGame,
    PlayLevel,
    HitPause(f32),
    NextLevel,
    Continue,
    GameOver,
    GameStart,
    GameStartRandom,
    VolumeMinus,
    VolumePlus,
    PauseMinus,
    PausePlus,
    LevelMinus,
    LevelPlus,
    StarsMinus,
    StarsPlus,
    Quit
}

pub enum GameType {
    Normal,
    Random
}

pub enum PlayerCommand {
    Up,
    Down,
    Left,
    Right,
    Fire,
    Escape,
    Select,
    Click(i32, i32),
    MouseMove(i32, i32)
}

pub struct Game {
    state: GameState,
    game_over_screen: GameOverScreen,
    game_play_screen: PlayLevelScreen,
    main_menu_screen: MainMenuScreen,
    pause_menu_screen: PauseMenuScreen,
    settings_menu_screen: SettingsMenuScreen,
    top_score_screen: TopScoreScreen,
    display_info: DisplayInfo,
    level_number: i32,
    start_level: i32,
    score: i32,
    game_type: GameType,
    config: Configuration,
    texture_sprites: Texture2D,
    texture_font: Texture2D
}

impl Game {

    pub fn new(display_info: &DisplayInfo, config: Configuration, shader_manager: &ShaderManager) -> Game {

        let mut level = Level::empty();
        let game_play_screen = PlayLevelScreen::new(&mut level, display_info, config.pause_on_hit, 0, config.stars, shader_manager);
        let main_menu_screen = MainMenuScreen::new(display_info, shader_manager);
        let pause_menu_screen = PauseMenuScreen::new(display_info, shader_manager);
        let settings_menu_screen = SettingsMenuScreen::new(display_info, config.volume, config.pause_on_hit, 1, config.stars, shader_manager);
        let top_score_screen = TopScoreScreen::new(shader_manager);

        //TODO: move this to some resource manager
        //load sprite texture
        let img = image::open("assets/sprites.png").unwrap().into_rgba8();
        let texture_sprites = Texture2D::new();
        texture_sprites.set_image_data(img.width() as i32, img.height() as i32, img.as_bytes());
        texture_sprites.set_filtering(gl::LINEAR, gl::LINEAR);

        //load font texture
        let tex_img = image::open("assets/bitmap_font.png").unwrap().into_rgba8();
        let texture_font = Texture2D::new();
        texture_font.set_image_data(tex_img.width() as i32, tex_img.height() as i32, tex_img.as_bytes());
        texture_font.set_filtering(gl::LINEAR, gl::LINEAR);

        Game {
            state: GameState::MainMenuScreen,
            game_over_screen: GameOverScreen::new(&display_info.game_area, c"", 0, 0, shader_manager),
            game_play_screen,
            main_menu_screen,
            pause_menu_screen,
            settings_menu_screen,
            top_score_screen,
            display_info: *display_info,
            level_number: 0,
            start_level: 1,
            score: 0,
            game_type: GameType::Normal,
            config,
            texture_sprites,
            texture_font,
        }
    }

    pub fn configuration(&self) -> Configuration {
        self.config.clone()
    }

    pub fn update(&mut self, commands: &[PlayerCommand], single_command: &Option<PlayerCommand>, delta_time: f32, audio_manager: &AudioManager, shader_manager: &ShaderManager) -> bool {

        //game state machine
        self.state = match self.state {
            GameState::MainMenuScreen => {
                self.settings_menu_screen.set_back_state(GameState::MainMenuScreen);
                self.main_menu_screen.update(single_command)
            },
            GameState::SettingsMenuScreen => {
                self.settings_menu_screen.update(single_command)
            },
            GameState::TopScoreScreen => {
                self.top_score_screen.update(single_command, &self.config.score_table, &self.display_info)
            },
            GameState::PlayLevel => {
                self.game_play_screen.update(commands, single_command, self.level_number, audio_manager, delta_time)
            },
            //store a new score into the score table and set state to a "win screen"
            GameState::WinGame => {
                let score = self.game_play_screen.score();
                self.config.push_score(score);
                self.game_over_screen = GameOverScreen::new(&self.display_info.game_area, c"You win the game!", self.level_number, score, shader_manager);
                // self.score = 0;
                GameState::GameOverScreen
            },
            //store a new score into the score table and set state to a "game over screen"
            GameState::GameOver => {
                let score = self.game_play_screen.score();
                self.config.push_score(score);
                self.game_over_screen = GameOverScreen::new(&self.display_info.game_area, c"Game Over!", self.level_number, score, shader_manager);
                // self.score = 0;
                GameState::GameOverScreen
            },
            GameState::GameOverScreen => {
                self.game_over_screen.update(single_command)
            },
            GameState::Quit => {
                self.state
            },
            GameState::GameStart => {
                self.level_number = self.start_level - 1;
                self.score = 0;
                self.game_type = GameType::Normal;
                self.game_play_screen.reset_score();
                GameState::NextLevel
            },
            GameState::GameStartRandom => {
                self.level_number = self.start_level - 1;
                self.score = 0;
                self.game_type = GameType::Random;
                self.game_play_screen.reset_score();
                GameState::NextLevel
            },
            GameState::Continue => {
                GameState::PlayLevel
            },
            GameState::PauseMenuScreen => {
                self.settings_menu_screen.set_back_state(GameState::PauseMenuScreen);
                self.pause_menu_screen.update(single_command)
            },
            //generates a new level (from the config or random one)
            GameState::NextLevel => {
                let monsters_str = match self.game_type {
                    GameType::Normal => {
                        match self.config.levels.get((self.level_number) as usize) {
                            Some(ls) => ls,
                            None => {
                                //when all level are done, you win the game
                                self.state = GameState::WinGame;
                                return false;
                            },
                        }
                    },
                    //random monsters for 5x11 grid
                    GameType::Random => "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
                };

                //increae a level number, create a new level from template string and apply level modifications
                self.level_number += 1;
                let mut level = Level::create(monsters_str);
                level.set_monster_laser_speed_level(self.level_number);
                level.set_monster_laser_timer_level(self.level_number);
                level.set_monster_speed_level(self.level_number);
                level.set_ship_laser_timer_level(self.level_number);

                //every 20th levels +1 live
                let extra_lives = self.level_number / 20;
                level.set_ship_lives(SHIP_INIT_LIVES + extra_lives);

                //update score
                self.score = self.game_play_screen.score();
                self.game_play_screen = PlayLevelScreen::new(&mut level, &self.display_info, self.config.pause_on_hit, self.score, self.config.stars, shader_manager);
                GameState::PlayLevel
            },
            //when player is hit pause a gem for a while
            GameState::HitPause(t) => {
                if self.config.pause_on_hit > 0.0 && t - delta_time > 0.0 {
                    GameState::HitPause(t - delta_time)
                } else {
                    self.game_play_screen.update(commands, single_command, self.level_number, audio_manager, delta_time)
                }
            },
            GameState::VolumePlus => {
                let v = self.config.volume + 5;
                self.config.volume = i32::min(v, 100);
                self.settings_menu_screen.set_volume(self.config.volume);
                audio_manager.set_volume(self.config.volume);
                audio_manager.play_sound(SoundType::ShipFire);
                GameState::SettingsMenuScreen
            },
            GameState::VolumeMinus => {
                let v = self.config.volume - 5;
                self.config.volume = i32::max(v, 0);
                self.settings_menu_screen.set_volume(self.config.volume);
                audio_manager.set_volume(self.config.volume);
                audio_manager.play_sound(SoundType::ShipFire);
                GameState::SettingsMenuScreen
            },
            GameState::PausePlus => {
                let v = self.config.pause_on_hit + 0.1;
                self.config.pause_on_hit = f32::min(v, 5.0);
                self.settings_menu_screen.set_pause(self.config.pause_on_hit);
                self.game_play_screen.set_pause_on_hit(self.config.pause_on_hit);
                GameState::SettingsMenuScreen
            },
            GameState::PauseMinus => {
                let v = self.config.pause_on_hit - 0.1;
                self.config.pause_on_hit = f32::max(v, 0.0);
                self.settings_menu_screen.set_pause(self.config.pause_on_hit);
                self.game_play_screen.set_pause_on_hit(self.config.pause_on_hit);
                GameState::SettingsMenuScreen
            },
            GameState::LevelPlus => {
                self.start_level = i32::min(self.start_level + 1, self.config.levels.len() as i32);
                self.settings_menu_screen.set_level(self.start_level);
                GameState::SettingsMenuScreen
            },
            GameState::LevelMinus => {
                self.start_level = i32::max(self.start_level - 1, 1);
                self.settings_menu_screen.set_level(self.start_level);
                GameState::SettingsMenuScreen
            },
            GameState::StarsPlus => {
                let v = self.config.stars + 10;
                self.config.stars = i32::min(v, STAR_MAX_COUNT);
                self.settings_menu_screen.set_stars(self.config.stars);
                GameState::SettingsMenuScreen
            },
            GameState::StarsMinus => {
                let v = self.config.stars - 10;
                self.config.stars = i32::max(v, 0);
                self.settings_menu_screen.set_stars(self.config.stars);
                GameState::SettingsMenuScreen
            },
        };

        self.state == GameState::Quit
    }

    pub fn render(&mut self, shader_manager: &ShaderManager, delta_time: f32) {

        //activate texture units before rendering
        self.texture_sprites.activate(TEXTURE_UNIT_SPRITE_ATLAS);
        self.texture_font.activate(TEXTURE_UNIT_BITMAP_FONT);

        match self.state {
            GameState::GameOverScreen => self.game_over_screen.render(shader_manager),
            GameState::MainMenuScreen => self.main_menu_screen.render(shader_manager),
            GameState::PauseMenuScreen => self.pause_menu_screen.render(shader_manager),
            GameState::TopScoreScreen => self.top_score_screen.render(shader_manager),
            GameState::SettingsMenuScreen => self.settings_menu_screen.render(shader_manager),
            GameState::VolumePlus | GameState::VolumeMinus => self.settings_menu_screen.render(shader_manager),
            GameState::PausePlus | GameState::PauseMinus => self.settings_menu_screen.render(shader_manager),
            GameState::LevelPlus | GameState::LevelMinus => self.settings_menu_screen.render(shader_manager),
            GameState::StarsPlus | GameState::StarsMinus => self.settings_menu_screen.render(shader_manager),
            GameState::PlayLevel | GameState::HitPause(_) => self.game_play_screen.render(shader_manager, delta_time),
            _ => (),
        }
    }
}
