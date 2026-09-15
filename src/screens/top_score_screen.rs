use glam::{Vec2, Vec3};
use std::ffi::CString;
use std::i32;
use crate::bitmap_string::{BitmapGlyphGroup, BitmapStringRenderer, CHAR_GRID_HEIGHT, CHAR_GRID_WIDTH};
use crate::common::DisplayInfo;
use crate::constants::{FONT_COLOR, GAME_AREA_WIDTH, TOP_SCORE_COUNT};
use crate::game::{GameState, PlayerCommand};
use crate::shader_manager::ShaderManager;


pub struct TopScoreScreen {
    score_table: Vec<i32>,
    bitmap_string: BitmapGlyphGroup,
    bitmap_string_renderer: BitmapStringRenderer
}

impl TopScoreScreen {

    pub fn new(shader_manager: &ShaderManager) -> TopScoreScreen {
        //just empty screen. Everything is "updated" when necessary
        TopScoreScreen {
            score_table: Vec::new(),
            bitmap_string: BitmapGlyphGroup::new(),
            bitmap_string_renderer: BitmapStringRenderer::new(shader_manager),
        }
    }

    pub fn update(&mut self, single_command: &Option<PlayerCommand>, score_table: &Vec<i32>, display_info: &DisplayInfo) -> GameState {

        //update score table and generate a glyphs if changed
        if *score_table != self.score_table {
            self.score_table = score_table.clone();

            //top score string
            self.bitmap_string.clear();
            let score_cstring = c"top score:";
            let mut pos = Vec2 {
                x: display_info.game_area.left + (GAME_AREA_WIDTH - score_cstring.count_bytes() as f32 * CHAR_GRID_WIDTH as f32) / 2.0,
                y: display_info.game_area.top + CHAR_GRID_HEIGHT as f32
            };
            self.bitmap_string.add_glyphs(score_cstring, &pos, &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &FONT_COLOR);

            //table of 20 top scores. Skip 0 records
            pos.x = display_info.game_area.left + (GAME_AREA_WIDTH - 5.0 * CHAR_GRID_WIDTH as f32) / 2.0;
            pos.y += CHAR_GRID_HEIGHT as f32;
            score_table.iter()
                .take(TOP_SCORE_COUNT)
                .filter(|&&v| v != 0 )
                .for_each(|v| {
                    let score_cstring = CString::new(format!("{v:5}")).unwrap();
                    pos.y += CHAR_GRID_HEIGHT as f32;
                    self.bitmap_string.add_glyphs(&score_cstring, &pos, &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &Vec3::ONE);
                });
        }

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
        GameState::TopScoreScreen
    }

    pub fn render(&mut self, shader_manager: &ShaderManager) {
        self.bitmap_string_renderer.render(shader_manager, &self.bitmap_string);
    }
}
