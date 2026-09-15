use gl;
use gl::types::*;

pub struct Texture2D {
    id: GLuint,
}

impl Texture2D {

    pub fn new() -> Texture2D {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Texture2D{ id }
    }

    // pub fn bind(&self) {
    //     unsafe {
    //         gl::BindTexture(gl::TEXTURE_2D, self.id);
    //     }
    // }

    //TODO: make mipmapping as a separate method. And add level parameter
    // TODO: check data size!!
    // TODO: add texture unit here??!!
    // pub fn set_image_data(&self, level: GLint, width: GLsizei, height: GLsizei, data: &[u8]) {
    //load image into the texture. Image must be in RGBA [u8] format, and is stored as RGBA in graphic card
    pub fn set_image_data(&self, width: GLsizei, height: GLsizei, data: &[u8]) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr().cast());
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    //set wrapping mode for 2D texture
    //x_wrap should be like: GL_CLAMP_TO_EDGE, GL_CLAMP_TO_BORDER, GL_MIRRORED_REPEAT, GL_REPEAT, GL_MIRROR_CLAMP_TO_EDGE
    pub fn set_wrapping(&self, s_wrap: GLuint, t_wrap: GLuint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, s_wrap as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, t_wrap as GLint);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    //set filtering mode for 2D texture
    //min_filter: GL_NEAREST, GL_LINEAR, GL_NEAREST_MIPMAP_NEAREST, GL_LINEAR_MIPMAP_NEAREST, GL_NEAREST_MIPMAP_LINEAR, GL_LINEAR_MIPMAP_LINEAR
    //max_filter: GL_NEAREST, GL_LINEAR
    pub fn set_filtering(&self, min_filter: GLuint, mag_filter: GLuint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as GLint);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    //set generic GLfloat 2D texture parameter
    pub fn set_parameter_f(&self, pname: GLenum, v: GLfloat) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterf(gl::TEXTURE_2D, pname, v);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    //set generic GLint 2D texture parameter
    pub fn set_parameter_i(&self, pname: GLenum, v: GLint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, pname, v);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    //set generic slice of GLfloat 2D texture parameters
    pub fn set_parameter_fv(&self, pname: GLenum, v: &[GLfloat]) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterfv(gl::TEXTURE_2D, pname, v.as_ptr() as *const _);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    //set generic slice of GLint 2D texture parameters
    pub fn set_parameter_iv(&self, pname: GLenum, v: &[GLint]) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteriv(gl::TEXTURE_2D, pname, v.as_ptr() as *const _);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn activate(&self, texture_unit: GLuint) {
        unsafe {
            gl::ActiveTexture(texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture2D {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
