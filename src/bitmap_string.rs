use gl::types::{GLint, GLsizei, GLuint};
use glam::{Vec2, Vec3, Vec4};
use std::ffi::CStr;
use crate::constants::*;
use crate::ogl::buffer::VertexBuffer;
use crate::ogl::vertex_array::VertexArray;
use crate::shader_manager::{ShaderManager, ShaderType};


//THIS MUST BE IN SYNC WITH BITMAP FONT TEXTURE. Dimensions are in pixels.
const CHAR_MAP: &CStr = c" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
pub const CHAR_GRID_WIDTH: u32 = 30;
pub const CHAR_GRID_HEIGHT: u32 = 30;
const CHARS_PER_ROW: u32 = 19;
const CHARS_PER_COL: u32 = 5;

const CHAR_TEXTURE_WIDTH: f32 = 1.0 / CHARS_PER_ROW as f32;
const CHAR_TEXTURE_HEIGHT: f32 = 1.0 / CHARS_PER_COL as f32;


//get character index from character map
fn glyph_index(char: u8) -> u32 {
    let (idx, _) = CHAR_MAP.to_bytes().iter().enumerate().find(|(_, b)| **b == char).expect("character not found in character map!");
    idx as u32
}

pub struct BitmapGlyph {
    position: Vec2,
    scale: Vec2,
    texture_coordinate: Vec2,
    color: Vec3
}

impl BitmapGlyph {

    pub fn new(char: u8, position: &Vec2, scale: &Vec2, color: &Vec3) -> BitmapGlyph {
        let idx = glyph_index(char);
        let col = idx % CHARS_PER_ROW;
        let row = idx / CHARS_PER_ROW;
        let texture_x = (col * CHAR_GRID_WIDTH) as f32 / (CHARS_PER_ROW * CHAR_GRID_WIDTH) as f32;
        let texture_y = (row * CHAR_GRID_HEIGHT) as f32 / (CHARS_PER_COL * CHAR_GRID_HEIGHT) as f32;

        BitmapGlyph {
            position: *position,
            scale: *scale,
            texture_coordinate: Vec2::new(texture_x, texture_y),
            color: *color
        }
    }

    pub fn set_char(&mut self, char: u8) {
        let idx = glyph_index(char);
        let col = idx % CHARS_PER_ROW;
        let row = idx / CHARS_PER_ROW;
        let texture_x = (col * CHAR_GRID_WIDTH) as f32 / (CHARS_PER_ROW * CHAR_GRID_WIDTH) as f32;
        let texture_y = (row * CHAR_GRID_HEIGHT) as f32 / (CHARS_PER_COL * CHAR_GRID_HEIGHT) as f32;
        self.texture_coordinate = Vec2::new(texture_x, texture_y);
    }
}


pub struct BitmapGlyphGroup {
    glyphs: Vec<BitmapGlyph>
}

impl BitmapGlyphGroup {

    pub fn new() -> BitmapGlyphGroup {
        BitmapGlyphGroup {
            glyphs: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.glyphs.clear();
    }

    //return index of the string that ca be used in "replace string" function
    pub fn add_glyphs(&mut self, cstr: &CStr, position: &Vec2, scale: &Vec2, color: &Vec3) -> usize {
        let start_idx = self.glyphs.len();

        cstr.to_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, b)| {
                let mut p = *position;
                p.x += i as f32 * scale.x;
                self.glyphs.push(BitmapGlyph::new(*b, &p, scale, color));
            });
        start_idx
    }

    pub fn replace_glyphs_chars(&mut self, index: usize, cstr: &CStr) {
        cstr.to_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, b)| {
                self.glyphs[i + index].set_char(*b);
            });
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BitmapGlyph> {
        self.glyphs.iter()
    }
}


pub struct BitmapStringRenderer {
    vao: VertexArray,
    vbo_matrices: VertexBuffer,
    vbo_colors: VertexBuffer,
    vbo_texture_offsets: VertexBuffer,
    color_location: GLint,
    texture_location: GLuint,
    colors: Vec<Vec3>,
    texture_offset_location: GLint,
    texture_offsets: Vec<Vec2>,
    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>
}

impl BitmapStringRenderer {

    pub fn new(shader_manager: &ShaderManager) -> BitmapStringRenderer {
        let shader_program = shader_manager.shader(ShaderType::BitmapStringShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //VBO for instanced colors, texture offsets and model/world matrices
        let vbo_colors = VertexBuffer::new();
        let vbo_texture_offsets = VertexBuffer::new();
        let vbo_matrices = VertexBuffer::new();

        //location of shader input parameters
        let texture_location = shader_program.uniform_location(c"texture_font").unwrap();
        let texture_offset_location = shader_program.attribute_location(c"tex_offset").unwrap();
        let color_location = shader_program.attribute_location(c"color").unwrap();
        let model_pos_scale_location = shader_program.attribute_location(c"m_pos_scale").unwrap();

        //quad with position and base texture coordinates
        let vertices: [f32; 24] = [
            0.0, 0.0, 0.0, 0.0,
            1.0, 0.0, CHAR_TEXTURE_WIDTH, 0.0,
            1.0, 1.0, CHAR_TEXTURE_WIDTH, CHAR_TEXTURE_HEIGHT,
            1.0, 1.0, CHAR_TEXTURE_WIDTH, CHAR_TEXTURE_HEIGHT,
            0.0, 1.0, 0.0, CHAR_TEXTURE_HEIGHT,
            0.0, 0.0, 0.0, 0.0,
        ];

        let vao = VertexArray::new();
        let vbo = VertexBuffer::new();
        vbo.buffer_data(&vertices, gl::STATIC_DRAW);

        vao.bind();
        vbo.bind();
        vbo.set_attribute_pointer(0, 2, gl::FLOAT, 4 * size_of::<f32>() as GLint, 0, 0);
        vbo.set_attribute_pointer(1, 2, gl::FLOAT, 4 * size_of::<f32>() as GLint, 2 * size_of::<f32>() as GLint, 0);
        vbo.unbind();
        vao.unbind();

        BitmapStringRenderer {
            vao,
            vbo_matrices,
            vbo_colors,
            vbo_texture_offsets,
            texture_location,
            color_location,
            colors: Vec::new(),
            texture_offset_location,
            texture_offsets: Vec::new(),
            model_pos_scale_location,
            model_pos_scale: Vec::new()
        }
    }

    pub fn render(&mut self, shader_manager: &ShaderManager, glyph_group: &BitmapGlyphGroup) {

        //update list of all model matrices texture offsets and colors.
        // TODO: this is almost static. No need to recreate it every time
        self.colors.clear();
        self.texture_offsets.clear();
        self.model_pos_scale.clear();
        self.model_pos_scale.extend(
            glyph_group.iter()
                .map(|g| {
                    let p = g.position;
                    let s = g.scale;
                    self.colors.push(g.color);
                    self.texture_offsets.push(g.texture_coordinate);
                    Vec4::new(p.x, p.y, s.x, s.y)
                }));

        self.vao.bind();
        self.vbo_colors.buffer_data(&self.colors, gl::STATIC_DRAW);
        self.vbo_colors.set_attribute_pointer(self.color_location as GLuint, 3, gl::FLOAT, size_of::<Vec3>() as GLint, 0, 1);

        self.vbo_texture_offsets.buffer_data(&self.texture_offsets, gl::STATIC_DRAW);
        self.vbo_texture_offsets.set_attribute_pointer(self.texture_offset_location as GLuint, 2, gl::FLOAT, size_of::<Vec2>() as GLint, 0, 1);

        self.vbo_matrices.buffer_data(&self.model_pos_scale, gl::STATIC_DRAW);
        self.vbo_matrices.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);

        let shader_program = shader_manager.shader(ShaderType::BitmapStringShader).unwrap();
        shader_program.use_it();
        shader_program.set_uniform_i(self.texture_location, TEXTURE_UNIT_BITMAP_FONT_UNIFORM_IDX);

        unsafe {
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.model_pos_scale.len() as GLsizei);
        }
        self.vao.unbind();
    }
}
