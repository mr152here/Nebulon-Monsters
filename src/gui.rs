use gl::types::{GLint, GLuint};
use glam::{Mat4, Vec2, Vec3, Vec4};
use std::ffi::CString;
use crate::bitmap_string::{BitmapGlyphGroup, BitmapStringRenderer, CHAR_GRID_HEIGHT, CHAR_GRID_WIDTH};
use crate::common::GameArea;
use crate::constants::*;
use crate::ogl::buffer::VertexBuffer;
use crate::ogl::vertex_array::VertexArray;
use crate::shader_manager::{ShaderManager, ShaderType};


//simple GUI around the play area. Simple frame, level number, score, and player lives
pub struct Gui {
    bitmap_string: BitmapGlyphGroup,
    game_area: GameArea,
    score: i32,
    level: i32,
    lives: i32
}

impl Gui {

    pub fn new(game_area: &GameArea) -> Gui {
        let bitmap_string = BitmapGlyphGroup::new();
        Gui {
            bitmap_string,
            game_area: *game_area,
            score: -1,
            level: -1,
            lives: 0
        }
    }

    fn strings(&self) -> &BitmapGlyphGroup {
        &self.bitmap_string
    }

    pub fn update(&mut self, level: i32, score: i32, lives: i32) {
        if self.level != level || self.score != score {
            self.level = level;
            self.score = score;

            let score_cstring = CString::new(format!("Level:{level:02}       Score:{score:5}")).unwrap();
            self.bitmap_string.clear();
            self.bitmap_string.add_glyphs(&score_cstring, &Vec2::new(self.game_area.left, self.game_area.top - CHAR_GRID_HEIGHT as f32), &Vec2::new(CHAR_GRID_WIDTH as f32, CHAR_GRID_HEIGHT as f32), &Vec3::ONE);
        }
        self.lives = lives;
    }
}

pub struct GuiRenderer {
    vao_frame: VertexArray,
    vao_lives: VertexArray,
    vbo_lives_models: VertexBuffer,
    model_frame_location: GLuint,
    model_frame: Mat4,
    color_lives_location: GLuint,
    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>,
    texture_location: GLuint,
    bitmap_string: BitmapStringRenderer
}

impl GuiRenderer {

    pub fn new(game_area: &GameArea, shader_manager: &ShaderManager) -> GuiRenderer {

        let shader_frame = shader_manager.shader(ShaderType::FrameShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_frame.uniform_block_index(c"Matrices").unwrap();
        shader_frame.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //get location of model matrix
        let model_frame_location = shader_frame.uniform_location(c"model").unwrap();

        //line strip - 4 lines
        let vertices_frame: [f32; 10] = [
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0
        ];

        let model_matrix_frame = Mat4::from_translation(Vec3 {x: game_area.left, y: game_area.top, z: 0.0}) * Mat4::from_scale(Vec3::new(GAME_AREA_WIDTH, GAME_AREA_HEIGHT, 0.0));

        //create shader program for lives
        let shader_lives = shader_manager.shader(ShaderType::LivesShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_lives.uniform_block_index(c"Matrices").unwrap();
        shader_lives.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //get location of model matrix, color and texture sprites
        let model_pos_scale_location = shader_lives.attribute_location(c"m_pos_scale").unwrap();
        let color_lives_location = shader_lives.uniform_location(c"color").unwrap();
        let texture_location = shader_lives.uniform_location(c"texture_sprites").unwrap();

        let vertices_lives: [f32; 24] = [
            0.0, 0.0, 0.0, 1.0 - TEXTURE_ATLAS_V_STEP,
            1.0, 0.0, 0.5, 1.0 - TEXTURE_ATLAS_V_STEP,
            1.0, 1.0, 0.5, 1.0,
            1.0, 1.0, 0.5, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 0.0, 1.0 - TEXTURE_ATLAS_V_STEP,
        ];

        let vao_frame = VertexArray::new();
        let vbo_frame = VertexBuffer::new();
        vbo_frame.buffer_data(&vertices_frame, gl::STATIC_DRAW);

        vao_frame.bind();
        vbo_frame.bind();
        vbo_frame.set_attribute_pointer(0, 2, gl::FLOAT, 2 * size_of::<f32>() as GLint, 0, 0);
        vbo_frame.unbind();
        vao_frame.unbind();

        let vao_lives = VertexArray::new();
        let vbo_lives = VertexBuffer::new();
        vbo_lives.buffer_data(&vertices_lives, gl::STATIC_DRAW);

        vao_lives.bind();
        vbo_lives.bind();
        vbo_lives.set_attribute_pointer(0, 2, gl::FLOAT, 4 * size_of::<f32>() as GLint, 0, 0);
        vbo_lives.set_attribute_pointer(1, 2, gl::FLOAT, 4 * size_of::<f32>() as GLint, 2 * size_of::<f32>() as GLint, 0);
        vbo_lives.unbind();
        vao_lives.unbind();

        //generate positions and scales for max 10 lives
        //TODO: this will fail if someone get more than 10 lives (currently at level 140+)
        let model_pos_scale = vec![
            Vec4::new(game_area.left + 0.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 1.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 2.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 3.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 4.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 5.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 6.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 7.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 8.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT),
            Vec4::new(game_area.left + 9.2 * (SHIP_WIDTH + 14.0), game_area.bottom + SHIP_HEIGHT * 0.5, SHIP_WIDTH, SHIP_HEIGHT)
        ];

        GuiRenderer {
            vao_frame,
            vao_lives,
            vbo_lives_models: VertexBuffer::new(), 
            model_frame_location,
            color_lives_location,
            model_pos_scale_location,
            model_pos_scale,
            bitmap_string: BitmapStringRenderer::new(shader_manager),
            model_frame: model_matrix_frame,
            texture_location
        }
    }

    pub fn render(&mut self, shader_manager: &ShaderManager, gui: &Gui) {

        //render level and score
        self.bitmap_string.render(shader_manager, gui.strings());

        //render frame
        let shader_frame = shader_manager.shader(ShaderType::FrameShader).unwrap();
        shader_frame.use_it();
        shader_frame.set_uniform_mat4v(self.model_frame_location, &[self.model_frame]);

        self.vao_frame.bind();
        unsafe {
            gl::DrawArrays(gl::LINE_STRIP, 0, 5);
        }
        self.vao_frame.unbind();

        //render lives
        if gui.lives > 0 {

            let shader_lives = shader_manager.shader(ShaderType::LivesShader).unwrap();
            shader_lives.use_it();
            shader_lives.set_uniform_i(self.texture_location, TEXTURE_UNIT_SPRITE_ATLAS_UNIFORM_IDX);
            shader_lives.set_uniform_vec3(self.color_lives_location, &SHIP_COLOR);

            self.vao_lives.bind();
            self.vbo_lives_models.buffer_data(&self.model_pos_scale, gl::DYNAMIC_DRAW);
            self.vbo_lives_models.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);

            unsafe {
                gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, gui.lives);
            }
            self.vao_lives.unbind();
        }
    }
}
