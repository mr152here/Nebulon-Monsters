use gl;
use glam::{UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use gl::types::*;
use std::ffi::{CStr, CString};


pub struct ShaderProgram {
    pub id: GLuint
}

impl ShaderProgram {

    pub fn from_sources(vertex_shader: &CStr, fragment_shader: &CStr, geometry_shader: Option<&CStr>) -> Result<ShaderProgram, String> {

        let vs = match Shader::from_source(vertex_shader, gl::VERTEX_SHADER) {
            Ok(vs) => vs,
            Err(s) => return Err(s)
        };

        let fs = match Shader::from_source(fragment_shader, gl::FRAGMENT_SHADER) {
            Ok(fs) => fs,
            Err(s) => return Err(s)
        };

        let gs = match geometry_shader {
            Some(gs) => match Shader::from_source(gs, gl::GEOMETRY_SHADER) {
                Ok(gs) => Some(gs),
                Err(s) => return Err(s)
            },
            None => None
        };

        //create program, link shaders and check for result
        let mut success = 1;
        let id = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vs.id);
            gl::AttachShader(id, fs.id);

            if let Some(ref gs) = gs {
                gl::AttachShader(id, gs.id);
            }

            gl::LinkProgram(id);
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            id
        };

        //if any error
        if success == 0 {
            let mut error_len = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut error_len);
            }

            let error_buffer = vec![b' '; error_len as usize + 1];
            let error_string = unsafe { CString::from_vec_unchecked(error_buffer) };
            unsafe {
                gl::GetProgramInfoLog(id, error_len, std::ptr::null_mut(), error_string.as_ptr() as *mut GLchar);
            }

            return Err(error_string.to_string_lossy().into_owned());
        }

        //detach all shaders
        unsafe {
            gl::DetachShader(id, vs.id);
            gl::DetachShader(id, fs.id);

            if let Some(gs) = gs {
                gl::DetachShader(id, gs.id);
            }
        }

        Ok(ShaderProgram { id })
    }

    pub fn use_it(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn attribute_location(&self, name: &CStr) -> Option<GLint> {
        unsafe {
            let i = gl::GetAttribLocation(self.id, name.as_ptr());
            if i != -1 {
                return Some(i);
            }
        }
        None
    }

    pub fn uniform_location(&self, name: &CStr) -> Option<GLuint> {
        unsafe {
            let i = gl::GetUniformLocation(self.id, name.as_ptr());
            if i != -1 {
                return Some(i as GLuint);
            }
        }
        None
    }

    // TODO: glGetUniformIndices for getting offsets of variables in uniform buffers
    // 

    pub fn uniform_block_binding(&self, block_index: GLuint, binding_point: GLuint) {
        unsafe {
            gl::UniformBlockBinding(self.id, block_index, binding_point);
        }
    }

    pub fn uniform_block_index(&self, name: &CStr) -> Option<GLuint> {
        unsafe {
            // let i = gl::GetUniformBlockIndex(self.id, name.as_ptr());
            let i = gl::GetProgramResourceIndex(self.id, gl::UNIFORM_BLOCK, name.as_ptr());
            if i != gl::INVALID_INDEX {
                return Some(i);
            }
        }
        None
    }

    pub fn storage_block_binding(&self, block_index: GLuint, binding_point: GLuint) {
        unsafe {
            gl::ShaderStorageBlockBinding(self.id, block_index, binding_point);
        }
    }

    pub fn storage_block_index(&self, name: &CStr) -> Option<GLuint> {
        unsafe {
            // let i = gl::GetUniformBlockIndex(self.id, name.as_ptr());
            let i = gl::GetProgramResourceIndex(self.id, gl::SHADER_STORAGE_BLOCK, name.as_ptr());
            if i != gl::INVALID_INDEX {
                return Some(i);
            }
        }
        None
    }

    // pub fn set_uniform_1f(&self, location: GLint, v0: GLfloat) {
    //     self.use_it();
    //     unsafe {
    //         gl::Uniform1f(location, v0);
    //     }
    // }

    pub fn set_uniform_i(&self, location: GLuint, v: GLint) {
        self.use_it();
        unsafe {
            gl::Uniform1i(location as GLint, v);
        }
    }

    pub fn set_uniform_vec2(&self, location: GLuint, v: &Vec2) {
        self.use_it();
        unsafe {
            gl::Uniform2f(location as GLint, v.x, v.y);
        }
    }

    pub fn set_uniform_vec3(&self, location: GLuint, v: &Vec3) {
        self.use_it();
        unsafe {
            gl::Uniform3f(location as GLint, v.x, v.y, v.z);
        }
    }

    pub fn set_uniform_vec4(&self, location: GLuint, v: &Vec4) {
        self.use_it();
        unsafe {
            gl::Uniform4f(location as GLint, v.x, v.y, v.z, v.w);
        }
    }

    pub fn set_uniform_uvec2(&self, location: GLuint, v: &UVec2) {
        self.use_it();
        unsafe {
            gl::Uniform2ui(location as GLint, v.x, v.y);
        }
    }

    pub fn set_uniform_uvec3(&self, location: GLuint, v: &UVec3) {
        self.use_it();
        unsafe {
            gl::Uniform3ui(location as GLint, v.x, v.y, v.z);
        }
    }

    pub fn set_uniform_uvec4(&self, location: GLuint, v: &UVec4) {
        self.use_it();
        unsafe {
            gl::Uniform4ui(location as GLint, v.x, v.y, v.x, v.w);
        }
    }

    pub fn set_uniform_mat3v(&self, location: GLuint, data: &[glam::Mat3]) {
        self.use_it();
        unsafe {
            gl::UniformMatrix3fv(location as GLint, data.len() as GLsizei, gl::FALSE, data.as_ptr() as *const _);
        }
    }

    pub fn set_uniform_mat4v(&self, location: GLuint, data: &[glam::Mat4]) {
        self.use_it();
        unsafe {
            gl::UniformMatrix4fv(location as GLint, data.len() as GLsizei, gl::FALSE, data.as_ptr() as *const _);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}


pub struct Shader {
    pub id: GLuint
}

impl Shader {

    pub fn from_source(source: &CStr, shader_type: GLuint) -> Result<Shader, String> {
    
        let id = unsafe { gl::CreateShader(shader_type) };
        let mut success = 1;

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        //check compilation status and returns error
        if success == 0 {

            let mut error_len = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_len);
            }

            let error_buffer = vec![b' '; error_len as usize + 1];
            let error_string = unsafe { CString::from_vec_unchecked(error_buffer) };
            unsafe {
                gl::GetShaderInfoLog(id, error_len, std::ptr::null_mut(), error_string.as_ptr() as *mut GLchar);
            }

            return Err(error_string.to_string_lossy().into_owned());
        }
        Ok(Shader { id })
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
