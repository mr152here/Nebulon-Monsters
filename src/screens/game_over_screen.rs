use glam::Vec2;
use std::ffi::{CStr, CString};
use crate::bitmap_string::{BitmapGlyphGroup, BitmapStringRenderer, CHAR_GRID_HEIGHT, CHAR_GRID_WIDTH};
use crate::common::GameArea;
use crate::constants::{FONT_COLOR, GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::game::{GameState, PlayerCommand};
use crate::shader_manager::ShaderManager;


pub struct GameOverScreen {
    bitmap_string: BitmapGlyphGroup,
    bitmap_string_renderer: BitmapStringRenderer
}

impl GameOverScreen {

    pub fn new(game_area: &GameArea, label: &CStr, level: i32, score: i32, shader_manager: &ShaderManager) -> GameOverScreen {
        let mut bitmap_string = BitmapGlyphGroup::new();

        //get Game Over! string into middle of the screen
        let mut pos = Vec2 {
            x: game_area.left + (GAME_AREA_WIDTH - label.count_bytes() as f32 * CHAR_GRID_WIDTH as f32) / 2.0,
            y: game_area.top + GAME_AREA_HEIGHT / 2.5
        };
        bitmap_string.add_glyphs(label, &pos, &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &FONT_COLOR);

        //place "level" string
        let level_cstring = CString::new(format!("Level:{level:02}")).unwrap();
        pos.y += 2.0 * CHAR_GRID_HEIGHT as f32;
        bitmap_string.add_glyphs(&level_cstring, &pos, &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &FONT_COLOR);

        //place "Score" string
        let score_cstring = CString::new(format!("Score:{score:05}")).unwrap();
        pos.y += CHAR_GRID_HEIGHT as f32;
        bitmap_string.add_glyphs(&score_cstring, &pos, &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &FONT_COLOR);

        let bitmap_string_renderer = BitmapStringRenderer::new(shader_manager);

        GameOverScreen {
            bitmap_string,
            bitmap_string_renderer,
        }
    }

    pub fn update(&mut self, single_command: &Option<PlayerCommand>) -> GameState {

        if let Some(command) = single_command {
            match command {
                PlayerCommand::Click(_, _) => {
                    return GameState::MainMenuScreen;
                },
                PlayerCommand::Select | PlayerCommand::Escape => {
                    return GameState::MainMenuScreen
                },
                _ => ()
            }
        }

        GameState::GameOverScreen
    }

    pub fn render(&mut self, shader_manager: &ShaderManager) {
        self.bitmap_string_renderer.render(shader_manager, &self.bitmap_string);
    }
}
