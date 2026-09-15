use gl::types::{GLint, GLsizei, GLuint};
use glam::{Mat4, Vec2, Vec3, Vec4};
use crate::common::GameArea;
use crate::shader_manager::{ShaderManager, ShaderType};
use crate::constants::*;
use crate::ogl::buffer::VertexBuffer;
use crate::ogl::vertex_array::VertexArray;
use crate::bomb::BombGroup;
use crate::block::BlockGroup;
use crate::laser::LaserGroup;
use crate::monster::{MonsterClass, MonsterGroup, MonsterState};
use crate::star::StarGroup;
use crate::ship::{Ship, ShipState};
use crate::ufo::{Ufo, UfoState};


const SHIP_SCALE_MATRIX: Mat4 = Mat4 { x_axis: Vec4::new(SHIP_WIDTH, 0.0, 0.0, 0.0), y_axis: Vec4::new(0.0, SHIP_HEIGHT, 0.0, 0.0), z_axis: Vec4::ZERO, w_axis: Vec4::W };
const UFO_SCALE_MATRIX: Mat4 = Mat4 { x_axis: Vec4::new(UFO_WIDTH, 0.0, 0.0, 0.0), y_axis: Vec4::new(0.0, UFO_HEIGHT, 0.0, 0.0), z_axis: Vec4::ZERO, w_axis: Vec4::W };

//simple renderer for blocks and lasers
pub struct SimpleRenderer {
    vao: VertexArray,
    vbo_matrices: VertexBuffer,
    vbo_colors: VertexBuffer,
    color_location: GLint,
    cover_model_pos_scale_l: Vec4,
    cover_model_pos_scale_r: Vec4,
    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>,
    colors: Vec<Vec3>
}

impl SimpleRenderer {

    pub fn new(shader_manager: &ShaderManager, game_area: &GameArea) -> SimpleRenderer {
        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //VBOs for instanced matrices and colors
        let vbo_matrices = VertexBuffer::new();
        let vbo_colors = VertexBuffer::new();
        
        //get location of model matrix and location for color attribute
        let color_location = shader_program.attribute_location(c"color").unwrap();
        let model_pos_scale_location = shader_program.attribute_location(c"m_pos_scale").unwrap();

        //simple quad with vec2 position 
        let vertices: [f32; 12] = [
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
        ];

        let vao = VertexArray::new();
        let vbo = VertexBuffer::new();
        vbo.buffer_data(&vertices, gl::STATIC_DRAW);

        vao.bind();
        vbo.bind();
        vbo.set_attribute_pointer(0, 2, gl::FLOAT, 2 * size_of::<f32>() as GLint, 0, 0);
        vbo.unbind();
        vao.unbind();

        let cover_model_pos_scale_l = Vec4::new(game_area.left - UFO_WIDTH, game_area.top, UFO_WIDTH, GAME_AREA_HEIGHT/8.0);
        let cover_model_pos_scale_r = Vec4::new(game_area.right, game_area.top, UFO_WIDTH, GAME_AREA_HEIGHT/8.0);

        SimpleRenderer {
            vao,
            vbo_matrices,
            vbo_colors,
            color_location,
            model_pos_scale_location,
            model_pos_scale: Vec::new(),
            cover_model_pos_scale_l,
            cover_model_pos_scale_r,
            colors: Vec::new()
        }
    }

    pub fn render(&mut self, shader_manager: &ShaderManager, laser_group: &LaserGroup, bomb_group: &BombGroup, block_group: &BlockGroup) {

        //update list of all positions and colors.
        self.model_pos_scale.clear();
        self.colors.clear();

        //add ufo covers
        self.model_pos_scale.push(self.cover_model_pos_scale_l);
        self.model_pos_scale.push(self.cover_model_pos_scale_r);
        self.colors.push(BACKGROUND_COLOR);
        self.colors.push(BACKGROUND_COLOR);

        //lasers
        self.model_pos_scale.extend(
            laser_group.iter()
                .filter_map(|l| {
                    if l.is_some() {
                        self.colors.push(LASER_COLOR);
                        let l = l.as_ref().unwrap();
                        let p = l.position();
                        return Some(Vec4::new(p.x, p.y, LASER_WIDTH, LASER_HEIGHT));
                    }
                    None
                }));

        //bombs
        self.model_pos_scale.extend(
            bomb_group.iter()
                .filter_map(|l| {
                    if l.is_some() {
                        self.colors.push(BOMB_COLOR);
                        let l = l.as_ref().unwrap();
                        let p = l.position();
                        return Some(Vec4::new(p.x, p.y, BOMB_WIDTH, BOMB_HEIGHT));
                    }
                    None
                }));

        //blocks (1 pixel shorter to create a little "grid" effect)
        // TODO: there is no need to recalculate this each frame. It doesn't change often
        self.model_pos_scale.extend(
            block_group.iter()
                .filter_map(|b| {
                    if b.is_some() {
                        self.colors.push(BLOCK_COLOR);
                        let l = b.as_ref().unwrap();
                        let p = l.position();
                        return Some(Vec4::new(p.x, p.y, BLOCK_WIDTH-1.0, BLOCK_HEIGHT-1.0));
                    }
                    None
                }));

        self.vao.bind();
        self.vbo_colors.buffer_data(&self.colors, gl::DYNAMIC_DRAW);
        self.vbo_colors.set_attribute_pointer(self.color_location as GLuint, 3, gl::FLOAT, size_of::<Vec3>() as GLint, 0, 1);

        self.vbo_matrices.buffer_data(&self.model_pos_scale, gl::DYNAMIC_DRAW);
        self.vbo_matrices.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);

        self.vao.unbind();

        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();
        shader_program.use_it();
        self.vao.bind();

        unsafe {
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.model_pos_scale.len() as GLsizei);
        }

        self.vao.unbind();
    }
}

pub struct StarRenderer {
    vao: VertexArray,
    vbo_matrices: VertexBuffer,
    vbo_colors: VertexBuffer,
    color_location: GLint,
    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>,
    colors: Vec<Vec3>
}

impl StarRenderer {

    pub fn new(shader_manager: &ShaderManager) -> StarRenderer {
        let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //VBOs for instanced matrices and colors
        let vbo_matrices = VertexBuffer::new();
        let vbo_colors = VertexBuffer::new();
        
        //get location of model matrix and location for color attribute
        let color_location = shader_program.attribute_location(c"color").unwrap();
        let model_pos_scale_location = shader_program.attribute_location(c"m_pos_scale").unwrap();

        //simple quad with vec2 position 
        let vertices: [f32; 12] = [
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
        ];

        let vao = VertexArray::new();
        let vbo = VertexBuffer::new();
        vbo.buffer_data(&vertices, gl::STATIC_DRAW);

        vao.bind();
        vbo.bind();
        vbo.set_attribute_pointer(0, 2, gl::FLOAT, 2 * size_of::<f32>() as GLint, 0, 0);
        vbo.unbind();
        vao.unbind();

        StarRenderer {
            vao,
            vbo_matrices,
            vbo_colors,
            color_location,
            model_pos_scale_location,
            model_pos_scale: Vec::new(),
            colors: Vec::new()
        }
    }

    pub fn render(&mut self, shader_manager: &ShaderManager, star_group: &StarGroup) {

        //update list of all positions and colors. If stars are enabled
        if star_group.count() > 0 {

            self.colors.clear();

            //transformation matrices are created in the shader. Just pass vec2 position and vec2 color for every star as one vec4
            //this is faster compared to sending precalculated matrices.
            self.model_pos_scale.clear();
            self.model_pos_scale.extend(
                star_group.iter()
                    .map(|s| {
                        self.colors.push(s.color());
                        let p = s.position();
                        let s = s.size();
                        Vec4::new(p.x, p.y, s.x, s.y)
                    }));

            self.vao.bind();
            self.vbo_colors.buffer_data(&self.colors, gl::DYNAMIC_DRAW);
            self.vbo_colors.set_attribute_pointer(self.color_location as GLuint, 3, gl::FLOAT, size_of::<Vec3>() as GLint, 0, 1);

            self.vbo_matrices.buffer_data(&self.model_pos_scale, gl::DYNAMIC_DRAW);
            self.vbo_matrices.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);
            self.vao.unbind();

            let shader_program = shader_manager.shader(ShaderType::SimpleShader).unwrap();
            shader_program.use_it();
            self.vao.bind();

            unsafe {
                gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.model_pos_scale.len() as GLsizei);
            }

            self.vao.unbind();
        }
    }
}


pub struct ShipRenderer {
    vao: VertexArray,
    model_matrix_location: GLuint,
    color_location: GLuint,
    texture_location: GLuint
}

impl ShipRenderer {

    pub fn new(shader_manager: &ShaderManager) -> ShipRenderer {

        let shader_program = shader_manager.shader(ShaderType::ShipShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //get location of model matrix
        let model_matrix_location = shader_program.uniform_location(c"model").unwrap();
        let color_location = shader_program.uniform_location(c"color").unwrap();
        let texture_location = shader_program.uniform_location(c"texture_sprites").unwrap();

        //quad with vec2 position and vec2 texture coordinates.
        //top left corner will be 0,0 after "flipped" orthogonal projection
        let vertices: [f32; 24] = [
            0.0, 0.0, 0.0, 1.0 - TEXTURE_ATLAS_V_STEP,
            1.0, 0.0, 0.5, 1.0 - TEXTURE_ATLAS_V_STEP,
            1.0, 1.0, 0.5, 1.0,
            1.0, 1.0, 0.5, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 0.0, 1.0 - TEXTURE_ATLAS_V_STEP,
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

        ShipRenderer {
            vao,
            model_matrix_location,
            color_location,
            texture_location
        }
    }

    pub fn render(&self, shader_manager: &ShaderManager, ship: &Ship) {

        let ship_position = ship.position();
        let color = match ship.state() {
            ShipState::Hit(_) => SHIP_COLOR_HIT,
            _ => SHIP_COLOR,
        };

        let model = Mat4::from_translation(Vec3 {x: ship_position.x, y: ship_position.y, z: 0.0}) * SHIP_SCALE_MATRIX;
        let shader_program = shader_manager.shader(ShaderType::ShipShader).unwrap();
        shader_program.use_it();
        shader_program.set_uniform_vec3(self.color_location, &color);
        shader_program.set_uniform_i(self.texture_location, TEXTURE_UNIT_SPRITE_ATLAS_UNIFORM_IDX);
        shader_program.set_uniform_mat4v(self.model_matrix_location, &[model]);
        self.vao.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        self.vao.unbind();
    }
}


pub struct MonsterRenderer {
    vao: VertexArray,
    vbo_matrices: VertexBuffer,
    vbo_colors: VertexBuffer,
    vbo_texture_offsets: VertexBuffer,
    color_location: GLint,
    colors: Vec<Vec3>,
    texture_location: GLuint,
    texture_offset_location: GLint,
    texture_offsets: Vec<Vec2>,

    model_pos_scale_location: GLint,
    model_pos_scale: Vec<Vec4>,
    animation_timer: f32,
    animation_frame: u32
}

impl MonsterRenderer {

    pub fn new(shader_manger: &ShaderManager) -> MonsterRenderer {
        let shader_program = shader_manger.shader(ShaderType::MonsterShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //VBO for instanced colors and model/world matrices
        let vbo_colors = VertexBuffer::new();
        let vbo_texture_offsets = VertexBuffer::new();
        let vbo_matrices = VertexBuffer::new();

        //uniform variables
        let texture_location = shader_program.uniform_location(c"texture_sprites").unwrap();
        
        //get location of color and model attributes. Those will change in every instance
        let texture_offset_location = shader_program.attribute_location(c"tex_offset").unwrap();
        let color_location = shader_program.attribute_location(c"color").unwrap();
        let model_pos_scale_location = shader_program.attribute_location(c"m_pos_scale").unwrap();

        //quad with vec2 position and vec2 base texture coordinates
        let vertices: [f32; 24] = [
            0.0, 0.0, 0.0, 0.0,
            1.0, 0.0, TEXTURE_ATLAS_H_STEP, 0.0,
            1.0, 1.0, TEXTURE_ATLAS_H_STEP, TEXTURE_ATLAS_V_STEP,
            1.0, 1.0, TEXTURE_ATLAS_H_STEP, TEXTURE_ATLAS_V_STEP,
            0.0, 1.0, 0.0, TEXTURE_ATLAS_V_STEP,
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

        MonsterRenderer {
            vao,
            vbo_matrices,
            vbo_colors,
            vbo_texture_offsets,
            color_location,
            colors: Vec::new(),
            texture_location,
            texture_offset_location,
            texture_offsets: Vec::new(),
            model_pos_scale_location,
            model_pos_scale: Vec::new(),
            animation_timer: ANIMATION_FRAME_TIMER,
            animation_frame: 0
        }
    }

    pub fn render(&mut self, shader_manger: &ShaderManager, monster_group: &MonsterGroup, delta_time: f32) {
        //update animations
        self.animation_timer -= delta_time;
        if self.animation_timer <= 0.0 {
            self.animation_timer += ANIMATION_FRAME_TIMER;
            self.animation_frame += 1;
            if self.animation_frame == MONSTERS_ANIMATION_FRAME_COUNT {
                self.animation_frame = 0;
            }
        }

        //update list of all model matrices, colors and texture coordinates.
        self.colors.clear();
        self.model_pos_scale.clear();
        self.texture_offsets.clear();
        //TODO: extend model_pos_scale
        monster_group.iter()
            .filter(|m| m.state() != MonsterState::Dead)
            .for_each(|m| {
                let mut to = Vec2::new(self.animation_frame as f32 * TEXTURE_ATLAS_H_STEP, 0.0);
                let hit = matches!(m.state(), MonsterState::Hit(_));

                let p = m.position();
                self.model_pos_scale.push(Vec4::new(p.x, p.y, MONSTER_WIDTH, MONSTER_HEIGHT));

                match m.class() {
                    MonsterClass::Fighter => {
                        self.colors.push(if hit { MONSTER_COLOR_HIT } else { MONSTER_COLOR_FIGHTER });
                        to += TEXTURE_ATLAS_FIGHTER_OFFSET;
                    },
                    MonsterClass::HeavyFighter => {
                        self.colors.push(if hit { MONSTER_COLOR_HIT } else { MONSTER_COLOR_HEAVY_FIGHTER });
                        to += TEXTURE_ATLAS_HEAVY_FIGHTER_OFFSET;
                    },
                    MonsterClass::Defender(_) => {
                        self.colors.push(if hit { MONSTER_COLOR_HIT } else { MONSTER_COLOR_DEFENDER });
                        to += TEXTURE_ATLAS_DEFENDER_OFFSET;
                    },
                    MonsterClass::Bomber => {
                        self.colors.push(if hit { MONSTER_COLOR_HIT } else { MONSTER_COLOR_BOMBER });
                        to += TEXTURE_ATLAS_BOMBER_OFFSET;
                    },
                    MonsterClass::Spawner => {
                        self.colors.push(if hit { MONSTER_COLOR_HIT } else { MONSTER_COLOR_SPAWNER });
                        to += TEXTURE_ATLAS_SPAWNER_OFFSET;
                    },
                }
                self.texture_offsets.push(to);
            });

        //if there is something to render
        if self.model_pos_scale.is_empty() {
            return;
        }

        self.vao.bind();
        self.vbo_colors.buffer_data(&self.colors, gl::DYNAMIC_DRAW);
        self.vbo_colors.set_attribute_pointer(self.color_location as GLuint, 3, gl::FLOAT, size_of::<Vec3>() as GLint, 0, 1);

        self.vbo_texture_offsets.buffer_data(&self.texture_offsets, gl::DYNAMIC_DRAW);
        self.vbo_texture_offsets.set_attribute_pointer(self.texture_offset_location as GLuint, 2, gl::FLOAT, size_of::<Vec2>() as GLint, 0, 1);

        self.vbo_matrices.buffer_data(&self.model_pos_scale, gl::DYNAMIC_DRAW);
        self.vbo_matrices.set_attribute_pointer(self.model_pos_scale_location as GLuint, 4, gl::FLOAT, size_of::<Vec4>() as GLint, 0, 1);

        let shader_program = shader_manger.shader(ShaderType::MonsterShader).unwrap();
        shader_program.use_it();
        shader_program.set_uniform_i(self.texture_location, TEXTURE_UNIT_SPRITE_ATLAS_UNIFORM_IDX);

        unsafe {
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.model_pos_scale.len() as GLsizei);
        }
        self.vao.unbind();
    }
}


pub struct UfoRenderer {
    vao: VertexArray,
    color_location: GLuint,
    texture_location: GLuint,
    texture_offset_location: GLuint,
    model_matrix_location: GLuint,
    animation_timer: f32,
    animation_frame: u32
}

impl UfoRenderer {

    pub fn new(shader_manager: &ShaderManager) -> UfoRenderer {

        let shader_program = shader_manager.shader(ShaderType::UfoShader).unwrap();

        //bind uniform buffer object for matrices
        let b_idx = shader_program.uniform_block_index(c"Matrices").unwrap();
        shader_program.uniform_block_binding(b_idx, UNIFORM_BUFFER_BINDING_MATRICES);

        //get uniform location of color texture/frame offset and model matrix
        let texture_location = shader_program.uniform_location(c"texture_sprites").unwrap();
        let color_location = shader_program.uniform_location(c"color").unwrap();
        let texture_offset_location = shader_program.uniform_location(c"tex_offset").unwrap();
        let model_matrix_location = shader_program.uniform_location(c"model").unwrap();

        //quad with vec2 position and vec2 texture coordinates.
        //top left corner will be 0,0 after "flipped" orthogonal projection
        let vertices: [f32; 24] = [
            0.0, 0.0, 0.0, 1.0 - TEXTURE_ATLAS_V_STEP * 2.0,
            1.0, 0.0, 0.5, 1.0 - TEXTURE_ATLAS_V_STEP * 2.0,
            1.0, 1.0, 0.5, 1.0 - TEXTURE_ATLAS_V_STEP,
            1.0, 1.0, 0.5, 1.0 - TEXTURE_ATLAS_V_STEP,
            0.0, 1.0, 0.0, 1.0 - TEXTURE_ATLAS_V_STEP,
            0.0, 0.0, 0.0, 1.0 - TEXTURE_ATLAS_V_STEP * 2.0,
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

        UfoRenderer {
            vao,
            color_location,
            texture_location,
            texture_offset_location,
            model_matrix_location,
            animation_timer: ANIMATION_FRAME_TIMER,
            animation_frame: 0
        }
    }

    pub fn render(&mut self, shader_manager: &ShaderManager, ufo: &Ufo, delta_time: f32) {

        let ufo_state = ufo.state();
        match ufo_state {
            UfoState::Active | UfoState::Hit(_) => {
                let ufo_position = ufo.position();
                let ufo_color = match ufo_state {
                    UfoState::Active => UFO_COLOR,
                    UfoState::Hit(_) => UFO_COLOR_HIT,
                    _ => UFO_COLOR,
                };

                self.animation_timer -= delta_time;
                if self.animation_timer <= 0.0 {
                    self.animation_timer += ANIMATION_FRAME_TIMER;
                    self.animation_frame += 1;
                    if self.animation_frame == UFO_ANIMATION_FRAME_COUNT {
                        self.animation_frame = 0;
                    }
                }

                //offset in texture atlas for animation
                let to = Vec2::new(self.animation_frame as f32 * TEXTURE_ATLAS_H_STEP * 2.0, 0.0);

                //transformation matrix for UFO
                let model = Mat4::from_translation(Vec3 {x: ufo_position.x, y: ufo_position.y, z: 0.0}) * UFO_SCALE_MATRIX;

                let shader_program = shader_manager.shader(ShaderType::UfoShader).unwrap();
                shader_program.use_it();
                shader_program.set_uniform_i(self.texture_location, TEXTURE_UNIT_SPRITE_ATLAS_UNIFORM_IDX);
                shader_program.set_uniform_vec3(self.color_location, &ufo_color);
                shader_program.set_uniform_vec2(self.texture_offset_location, &to);
                shader_program.set_uniform_mat4v(self.model_matrix_location, &[model]);
                self.vao.bind();

                unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 6); }
                self.vao.unbind();
            },
            _ => (),
        }
    }
}
