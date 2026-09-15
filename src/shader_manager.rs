use std::collections::HashMap;
use std::ffi::CStr;
use crate::ogl::shader::ShaderProgram;

#[derive(Clone,Eq,Hash,PartialEq)]
pub enum ShaderType {
    BitmapStringShader,
    FrameShader,
    MonsterShader,
    LivesShader,
    ShipShader,
    SimpleShader,
    UfoShader
}

//hashmap that stores and compile shaders
pub struct ShaderManager {
    shaders: HashMap<ShaderType, ShaderProgram>,
}

impl ShaderManager {

    pub fn new() -> ShaderManager {
        ShaderManager {
            shaders: HashMap::new()
        }
    }

    pub fn load_shader_from_cstr(&mut self, vertex_shader: &CStr, fragment_shader: &CStr, shader_type: ShaderType) {
        let shader_program = ShaderProgram::from_sources(vertex_shader, fragment_shader, None).unwrap();
        self.shaders.insert(shader_type, shader_program);
    }

    pub fn shader(&self, shader_type: ShaderType) -> Option<&ShaderProgram> {
        self.shaders.get(&shader_type)
    }
}
