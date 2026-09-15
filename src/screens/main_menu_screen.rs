use gl::types::{GLint, GLsizei, GLuint};
use glam::{Vec2, Vec3, Vec4};
use std::ffi::CString;
use crate::bitmap_string::{BitmapGlyphGroup, BitmapStringRenderer, CHAR_GRID_HEIGHT, CHAR_GRID_WIDTH};
use crate::common::{AABB, DisplayInfo};
use crate::constants::*;
use crate::game::{GameState, PlayerCommand};
use crate::ogl::buffer::VertexBuffer;
use crate::ogl::vertex_array::VertexArray;
use crate::shader_manager::{ShaderManager, ShaderType};

const BUTTON_WIDTH: f32 = 380.0;
const BUTTON_HEIGHT: f32 = 80.0;
const BUTTON_SPACING: f32 = 15.0;


pub struct MenuButton {
    position: Vec2,
    size: Vec2,
    value: GameState
}

impl MenuButton {

    fn new(position: Vec2, size: Vec2, value: GameState) -> MenuButton {
        MenuButton {
            position,
            size,
            value
        }
    }

    //return true when point is in button area
    fn is_over(&self, x: f32, y: f32) -> bool {
        let aabb = AABB::new(self.position.y, self.position.y + self.size.y, self.position.x, self.position.x + self.size.x);
        aabb.collide_with_point(x, y)
    }
}

pub struct MainMenuScreen {
    vao: VertexArray,
    vbo_colors: VertexBuffer,
    vbo_matrices: VertexBuffer,
    color_location: GLint,
    colors: Vec<Vec3>,
    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>,
    selected_idx: usize,
    buttons: Vec<MenuButton>,
    bitmap_string: BitmapGlyphGroup,
    bitmap_string_renderer: BitmapStringRenderer,
    display_scale_factor: (f32, f32)
}

impl MainMenuScreen {

    pub fn new(display_info: &DisplayInfo, shader_manager: &ShaderManager) -> MainMenuScreen {

        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //get location of color and model attributes. Those will change in every instance
        let color_location = shader_program.attribute_location(c"color").unwrap();
        let model_pos_scale_location = shader_program.attribute_location(c"m_pos_scale").unwrap();

        //quad with vec2 position 
        let vertices: [f32; 12] = [
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
        ];

        let vao = VertexArray::new();
        let vbo_colors = VertexBuffer::new();
        let vbo_matrices = VertexBuffer::new();
        let vbo = VertexBuffer::new();
        vbo.buffer_data(&vertices, gl::STATIC_DRAW);

        vao.bind();
        vbo.bind();
        vbo.set_attribute_pointer(0, 2, gl::FLOAT, 2 * size_of::<f32>() as GLint, 0, 0);
        vbo.unbind();
        vao.unbind();

        let game_area = display_info.game_area;
        let button_left = game_area.left + (game_area.right - game_area.left - BUTTON_WIDTH) / 2.0;
        let buttons = vec![
            MenuButton::new(Vec2::new(button_left, 1.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::GameStart),
            MenuButton::new(Vec2::new(button_left, 2.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::GameStartRandom),
            MenuButton::new(Vec2::new(button_left, 3.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::TopScoreScreen),
            MenuButton::new(Vec2::new(button_left, 4.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::SettingsMenuScreen),
            MenuButton::new(Vec2::new(button_left, 5.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::Quit),
        ];

        let labels = [
            c"new game",
            c"random",
            c"top score",
            c"settings",
            c"quit"
        ];

        //labels on buttons
        let mut bitmap_string = BitmapGlyphGroup::new();
        for (b, l) in buttons.iter().zip(labels.iter()) {
            let pos = b.position;
            bitmap_string.add_glyphs(l, &Vec2::new(pos.x + (BUTTON_HEIGHT - CHAR_GRID_WIDTH as f32) / 2.0, pos.y + (BUTTON_HEIGHT- CHAR_GRID_HEIGHT as f32) / 2.0), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &Vec3::ZERO);
        }

        //create a version string at bottom right corner
        let version_string = CString::new(format!("Nebulon Monsters {}", env!("CARGO_PKG_VERSION"))).unwrap();
        let version_string = &version_string.as_c_str();
        bitmap_string.add_glyphs(version_string, &Vec2::new(display_info.display_width - ((version_string.count_bytes() + 2)*CHAR_GRID_WIDTH as usize) as f32, display_info.display_height - (2*CHAR_GRID_HEIGHT) as f32), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &VERSION_COLOR);

        //collect positions and scales from buttons
        let model_pos_scale = buttons.iter()
            .map(|b| {
                Vec4::new(b.position.x, b.position.y, b.size.x, b.size.y)
            })
            .collect();

        //color matrix for buttons
        let colors = buttons.iter().enumerate().map(|(i,_)| {
                if i == 0 { MENU_SELECTED_COLOR } else { MENU_COLOR }
            })
            .collect();

        MainMenuScreen {
            vao,
            vbo_colors,
            vbo_matrices,
            color_location,
            colors,
            model_pos_scale_location,
            model_pos_scale,
            selected_idx: 0,
            buttons,
            bitmap_string,
            bitmap_string_renderer: BitmapStringRenderer::new(shader_manager),
            display_scale_factor: display_info.scale_factor,
        }
    }

    pub fn update(&mut self, single_command: &Option<PlayerCommand>) -> GameState {

        if let Some(command) = single_command {
            match command {
                PlayerCommand::Click(x, y) => {
                    //first rescale mouse coordinates 
                    let x: f32 = self.display_scale_factor.0 * (*x) as f32;
                    let y: f32 = self.display_scale_factor.1 * (*y) as f32;

                    //find button that was clicked (if any)
                    let button_idx = self.buttons.iter().enumerate().find_map(|(i,b)| {
                        if b.is_over(x, y) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    //return its stored value (game state)
                    if let Some(idx) = button_idx {
                        return self.buttons[idx].value;
                    }
                },
                PlayerCommand::MouseMove(x, y) => {
                    //first rescale mouse coordinates 
                    let x: f32 = self.display_scale_factor.0 * (*x) as f32;
                    let y: f32 = self.display_scale_factor.1 * (*y) as f32;

                    let button_idx = self.buttons.iter().enumerate().find_map(|(i,b)| {
                        if b.is_over(x, y) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    if let Some(idx) = button_idx && self.selected_idx != idx {
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Up => {
                    if self.selected_idx > 0 {
                        let idx = self.selected_idx - 1;
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Down => {
                    if self.selected_idx < self.buttons.len() - 1 {
                        let idx = self.selected_idx + 1;
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Select | PlayerCommand::Fire => {
                    return self.buttons[self.selected_idx].value;
                },
                PlayerCommand::Escape => {
                    return GameState::Quit;
                },
                _ => ()
            }
        }

        GameState::MainMenuScreen
    }

    pub fn render(&mut self, shader_manager: &ShaderManager) {
        self.vao.bind();
        self.vbo_colors.buffer_data(&self.colors, gl::DYNAMIC_DRAW);
        self.vbo_colors.set_attribute_pointer(self.color_location as GLuint, 3, gl::FLOAT, size_of::<Vec3>() as GLint, 0, 1);

        self.vbo_matrices.buffer_data(&self.model_pos_scale, gl::DYNAMIC_DRAW);
        self.vbo_matrices.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);

        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();
        shader_program.use_it();

        unsafe {
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.model_pos_scale.len() as GLsizei);
        }
        self.vao.unbind();

        self.bitmap_string_renderer.render(shader_manager, &self.bitmap_string);
    }
}


pub struct SettingsMenuScreen {
    vao: VertexArray,
    vbo_colors: VertexBuffer,
    vbo_matrices: VertexBuffer,
    color_location: GLint,
    colors: Vec<Vec3>,
    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>,
    selected_idx: usize,
    buttons: Vec<MenuButton>,
    bitmap_string: BitmapGlyphGroup,
    bitmap_string_renderer: BitmapStringRenderer,
    volume_idx: usize,
    pause_idx: usize,
    level_idx: usize,
    stars_idx: usize,
    display_scale_factor: (f32, f32)
}

impl SettingsMenuScreen {

    pub fn new(display_info: &DisplayInfo, volume: i32, pause: f32, level: i32, stars: i32, shader_manager: &ShaderManager) -> SettingsMenuScreen {

        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //get location of color and model attributes. Those will change in every instance
        let color_location = shader_program.attribute_location(c"color").unwrap();
        let model_pos_scale_location = shader_program.attribute_location(c"m_pos_scale").unwrap();

        //quad with vec2 position 
        let vertices: [f32; 12] = [
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
        ];

        let vao = VertexArray::new();
        let vbo_colors = VertexBuffer::new();
        let vbo_matrices = VertexBuffer::new();
        let vbo = VertexBuffer::new();
        vbo.buffer_data(&vertices, gl::STATIC_DRAW);

        vao.bind();
        vbo.bind();
        vbo.set_attribute_pointer(0, 2, gl::FLOAT, 2 * size_of::<f32>() as GLint, 0, 0);
        vbo.unbind();
        vao.unbind();

        let game_area = display_info.game_area;
        let button_left = game_area.left + (game_area.right - game_area.left - BUTTON_WIDTH) / 2.0;
        let buttons = vec![
            MenuButton::new(Vec2::new(button_left, 1.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::VolumeMinus),
            MenuButton::new(Vec2::new(button_left + BUTTON_SPACING + BUTTON_HEIGHT, 1.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::VolumePlus),
            MenuButton::new(Vec2::new(button_left, 2.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::PauseMinus),
            MenuButton::new(Vec2::new(button_left + BUTTON_SPACING + BUTTON_HEIGHT, 2.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::PausePlus),
            MenuButton::new(Vec2::new(button_left, 3.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::LevelMinus),
            MenuButton::new(Vec2::new(button_left + BUTTON_SPACING + BUTTON_HEIGHT, 3.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::LevelPlus),
            MenuButton::new(Vec2::new(button_left, 4.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::StarsMinus),
            MenuButton::new(Vec2::new(button_left + BUTTON_SPACING + BUTTON_HEIGHT, 4.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_HEIGHT, BUTTON_HEIGHT), GameState::StarsPlus),
            MenuButton::new(Vec2::new(button_left, 5.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::MainMenuScreen),
        ];

        let labels = [
            c"-", c"+",
            c"-", c"+",
            c"-", c"+",
            c"-", c"+",
            c"back"
        ];

        //labels on buttons
        let mut bitmap_string = BitmapGlyphGroup::new();
        for (b, l) in buttons.iter().zip(labels.iter()) {
            let pos = b.position;
            bitmap_string.add_glyphs(l, &Vec2::new(pos.x + (BUTTON_HEIGHT - CHAR_GRID_WIDTH as f32) / 2.0, pos.y + (BUTTON_HEIGHT- CHAR_GRID_HEIGHT as f32) / 2.0), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &Vec3::ZERO);
        }

        //create a version string at bottom right corner
        let version_string = CString::new(format!("Nebulon Monsters {}", env!("CARGO_PKG_VERSION"))).unwrap();
        let version_string = &version_string.as_c_str();
        bitmap_string.add_glyphs(version_string, &Vec2::new(display_info.display_width - ((version_string.count_bytes() + 2)*CHAR_GRID_WIDTH as usize) as f32, display_info.display_height - (2*CHAR_GRID_HEIGHT) as f32), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &VERSION_COLOR);

        //volume
        let volume_cstr = CString::new(format!("volume:{:>3}%", volume)).unwrap();
        let volume_idx = bitmap_string.add_glyphs(&volume_cstr, &Vec2::new(button_left +  3.0 * BUTTON_HEIGHT, 1.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top + (BUTTON_HEIGHT- CHAR_GRID_HEIGHT as f32) / 2.0), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &SCORE_COLOR);

        //on hit pause
        let pause_cstr = CString::new(format!("pause:{:>4.1}s", pause)).unwrap();
        let pause_idx = bitmap_string.add_glyphs(&pause_cstr, &Vec2::new(button_left + 3.0 * BUTTON_HEIGHT, 2.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top + (BUTTON_HEIGHT- CHAR_GRID_HEIGHT as f32) / 2.0), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &SCORE_COLOR);

        //starting level
        let level_cstr = CString::new(format!("level:{:>2}", level)).unwrap();
        let level_idx = bitmap_string.add_glyphs(&level_cstr, &Vec2::new(button_left + 3.0 * BUTTON_HEIGHT, 3.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top + (BUTTON_HEIGHT- CHAR_GRID_HEIGHT as f32) / 2.0), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &SCORE_COLOR);

        //number of stars
        let stars_cstr = CString::new(format!("stars:{:>5}", stars)).unwrap();
        let stars_idx = bitmap_string.add_glyphs(&stars_cstr, &Vec2::new(button_left + 3.0 * BUTTON_HEIGHT, 4.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top + (BUTTON_HEIGHT- CHAR_GRID_HEIGHT as f32) / 2.0), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &SCORE_COLOR);

        //collect positions and scales from buttons
        let model_pos_scale = buttons.iter()
            .map(|b| {
                Vec4::new(b.position.x, b.position.y, b.size.x, b.size.y)
            })
            .collect();

        //colors for buttons
        let colors = buttons.iter().enumerate().map(|(i,_)| {
                if i == 0 { MENU_SELECTED_COLOR } else { MENU_COLOR }
            })
            .collect();

        SettingsMenuScreen {
            vao,
            vbo_colors,
            vbo_matrices,
            color_location,
            colors,
            model_pos_scale_location,
            model_pos_scale,
            selected_idx: 0,
            buttons,
            bitmap_string,
            bitmap_string_renderer: BitmapStringRenderer::new(shader_manager),
            volume_idx,
            pause_idx,
            level_idx,
            stars_idx,
            display_scale_factor: display_info.scale_factor
        }
    }

    pub fn set_back_state(&mut self, state: GameState) {
        self.buttons.last_mut().unwrap().value = state;
    }

    pub fn set_volume(&mut self, volume: i32) {
        let volume_cstr = CString::new(format!("volume:{:>3}%", volume)).unwrap();
        self.bitmap_string.replace_glyphs_chars(self.volume_idx, &volume_cstr);
    }

    pub fn set_pause(&mut self, pause: f32) {
        let pause_cstr = CString::new(format!("pause:{:>4.1}s", pause)).unwrap();
        self.bitmap_string.replace_glyphs_chars(self.pause_idx, &pause_cstr);
    }

    pub fn set_level(&mut self, level: i32) {
        let level_cstr = CString::new(format!("level:{:>2}", level)).unwrap();
        self.bitmap_string.replace_glyphs_chars(self.level_idx, &level_cstr);
    }

    pub fn set_stars(&mut self, stars: i32) {
        let stars_cstr = CString::new(format!("stars:{:>5}", stars)).unwrap();
        self.bitmap_string.replace_glyphs_chars(self.stars_idx, &stars_cstr);
    }

    pub fn update(&mut self, single_command: &Option<PlayerCommand>) -> GameState {

        if let Some(command) = single_command {
            match command {
                PlayerCommand::Click(x, y) => {
                    //first rescale mouse coordinates 
                    let x: f32 = self.display_scale_factor.0 * (*x) as f32;
                    let y: f32 = self.display_scale_factor.1 * (*y) as f32;

                    //find button that was clicked (if any)
                    let button_idx = self.buttons.iter().enumerate().find_map(|(i,b)| {
                        if b.is_over(x, y) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    //return its stored value (game state)
                    if let Some(idx) = button_idx {
                        return self.buttons[idx].value;
                    }
                },
                PlayerCommand::MouseMove(x, y) => {
                    //first rescale mouse coordinates 
                    let x: f32 = self.display_scale_factor.0 * (*x) as f32;
                    let y: f32 = self.display_scale_factor.1 * (*y) as f32;

                    let button_idx = self.buttons.iter().enumerate().find_map(|(i,b)| {
                        if b.is_over(x, y) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    if let Some(idx) = button_idx && self.selected_idx != idx {
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Up => {
                    if self.selected_idx > 0 {
                        let idx = self.selected_idx - 1;
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Down => {
                    if self.selected_idx < self.buttons.len() - 1 {
                        let idx = self.selected_idx + 1;
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Select | PlayerCommand::Fire => {
                    return self.buttons[self.selected_idx].value;
                },
                PlayerCommand::Escape => {
                    return self.buttons.last().unwrap().value;
                },
                _ => ()
            }
        }
        GameState::SettingsMenuScreen
    }

    pub fn render(&mut self, shader_manager: &ShaderManager) {

        self.vao.bind();
        self.vbo_colors.buffer_data(&self.colors, gl::DYNAMIC_DRAW);
        self.vbo_colors.set_attribute_pointer(self.color_location as GLuint, 3, gl::FLOAT, size_of::<Vec3>() as GLint, 0, 1);

        self.vbo_matrices.buffer_data(&self.model_pos_scale, gl::DYNAMIC_DRAW);
        self.vbo_matrices.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);

        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();
        shader_program.use_it();

        unsafe {
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.model_pos_scale.len() as GLsizei);
        }
        self.vao.unbind();

        self.bitmap_string_renderer.render(shader_manager, &self.bitmap_string);
    }
}


pub struct PauseMenuScreen {
    vao: VertexArray,
    vbo_colors: VertexBuffer,
    vbo_matrices: VertexBuffer,
    color_location: GLint,
    colors: Vec<Vec3>,
    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>,
    selected_idx: usize,
    buttons: Vec<MenuButton>,
    bitmap_string: BitmapGlyphGroup,
    bitmap_string_renderer: BitmapStringRenderer,
    display_scale_factor: (f32, f32)
}

impl PauseMenuScreen {

    pub fn new(display_info: &DisplayInfo, shader_manager: &ShaderManager) -> PauseMenuScreen{

        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //get location of color and model attributes. Those will change in every instance
        let color_location = shader_program.attribute_location(c"color").unwrap();
        let model_pos_scale_location = shader_program.attribute_location(c"m_pos_scale").unwrap();

        //quad with vec2 position 
        let vertices: [f32; 12] = [
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
        ];

        let vao = VertexArray::new();
        let vbo_colors = VertexBuffer::new();
        let vbo_matrices = VertexBuffer::new();
        let vbo = VertexBuffer::new();
        vbo.buffer_data(&vertices, gl::STATIC_DRAW);

        vao.bind();
        vbo.bind();
        vbo.set_attribute_pointer(0, 2, gl::FLOAT, 2 * size_of::<f32>() as GLint, 0, 0);
        vbo.unbind();
        vao.unbind();

        let game_area = display_info.game_area;
        let button_left = game_area.left + (game_area.right - game_area.left - BUTTON_WIDTH) / 2.0;
        let buttons = vec![
            MenuButton::new(Vec2::new(button_left, 1.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::Continue),
            MenuButton::new(Vec2::new(button_left, 3.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::SettingsMenuScreen),
            MenuButton::new(Vec2::new(button_left, 4.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::MainMenuScreen),
            MenuButton::new(Vec2::new(button_left, 5.5 * (BUTTON_HEIGHT + BUTTON_SPACING) + game_area.top), Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT), GameState::Quit),
        ];

        let labels = [
            c"continue",
            c"settings",
            c"main menu",
            c"quit"
        ];

        //labels on buttons
        let mut bitmap_string = BitmapGlyphGroup::new();
        for (b, l) in buttons.iter().zip(labels.iter()) {
            let pos = b.position;
            bitmap_string.add_glyphs(l, &Vec2::new(pos.x + (BUTTON_HEIGHT - CHAR_GRID_WIDTH as f32) / 2.0, pos.y + (BUTTON_HEIGHT- CHAR_GRID_HEIGHT as f32) / 2.0), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &Vec3::ZERO);
        }

        //create a version string at bottom right corner
        let version_string = CString::new(format!("Nebulon Monsters {}", env!("CARGO_PKG_VERSION"))).unwrap();
        let version_string = &version_string.as_c_str();
        bitmap_string.add_glyphs(version_string, &Vec2::new(display_info.display_width - ((version_string.count_bytes() + 2)*CHAR_GRID_WIDTH as usize) as f32, display_info.display_height - (2*CHAR_GRID_HEIGHT) as f32), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &VERSION_COLOR);

        //collect positions and scales from buttons
        let model_pos_scale = buttons.iter()
            .map(|b| {
                Vec4::new(b.position.x, b.position.y, b.size.x, b.size.y)
            })
            .collect();

        //color matrix for buttons
        let colors = buttons.iter().enumerate().map(|(i,_)| {
                if i == 0 { MENU_SELECTED_COLOR } else { MENU_COLOR }
            })
            .collect();

        PauseMenuScreen {
            vao,
            vbo_colors,
            vbo_matrices,
            color_location,
            colors,
            model_pos_scale_location,
            model_pos_scale,
            selected_idx: 0,
            buttons,
            bitmap_string,
            bitmap_string_renderer: BitmapStringRenderer::new(shader_manager),
            display_scale_factor: display_info.scale_factor
        }
    }

    pub fn update(&mut self, single_command: &Option<PlayerCommand>) -> GameState {

        if let Some(command) = single_command {
            match command {
                PlayerCommand::Click(x, y) => {
                    //first rescale mouse coordinates 
                    let x: f32 = self.display_scale_factor.0 * (*x) as f32;
                    let y: f32 = self.display_scale_factor.1 * (*y) as f32;

                    //find button that was clicked (if any)
                    let button_idx = self.buttons.iter().enumerate().find_map(|(i,b)| {
                        if b.is_over(x as f32, y as f32) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    //return its stored value (game state)
                    if let Some(idx) = button_idx {
                        return self.buttons[idx].value;
                    }
                },
                PlayerCommand::MouseMove(x, y) => {
                    //first rescale mouse coordinates 
                    let x: f32 = self.display_scale_factor.0 * (*x) as f32;
                    let y: f32 = self.display_scale_factor.1 * (*y) as f32;

                    let button_idx = self.buttons.iter().enumerate().find_map(|(i,b)| {
                        if b.is_over(x, y) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    if let Some(idx) = button_idx && self.selected_idx != idx {
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Up => {
                    if self.selected_idx > 0 {
                        let idx = self.selected_idx - 1;
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Down => {
                    if self.selected_idx < self.buttons.len() - 1 {
                        let idx = self.selected_idx + 1;
                        self.colors[idx] = MENU_SELECTED_COLOR;
                        self.colors[self.selected_idx] = MENU_COLOR;
                        self.selected_idx = idx;
                    }
                },
                PlayerCommand::Select | PlayerCommand::Fire => {
                    return self.buttons[self.selected_idx].value;
                },
                PlayerCommand::Escape => {
                    return GameState::Continue;
                },
                _ => ()
            }
        }
        GameState::PauseMenuScreen
    }

    pub fn render(&mut self, shader_manager: &ShaderManager) {
        self.vao.bind();
        self.vbo_colors.buffer_data(&self.colors, gl::DYNAMIC_DRAW);
        self.vbo_colors.set_attribute_pointer(self.color_location as GLuint, 3, gl::FLOAT, size_of::<Vec3>() as GLint, 0, 1);
        
        self.vbo_matrices.buffer_data(&self.model_pos_scale, gl::DYNAMIC_DRAW);
        self.vbo_matrices.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);

        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();
        shader_program.use_it();

        unsafe {
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.model_pos_scale.len() as GLsizei);
        }
        self.vao.unbind();

        self.bitmap_string_renderer.render(shader_manager, &self.bitmap_string);
    }
}
