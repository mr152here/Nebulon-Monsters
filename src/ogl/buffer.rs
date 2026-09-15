use gl;
use gl::types::*;


pub struct IndicesBuffer {
    pub id: GLuint,
}

impl IndicesBuffer {

    pub fn new() -> IndicesBuffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        IndicesBuffer{ id }
    }

    pub fn buffer_data<D: Sized>(&self, data: &[D], usage: GLenum) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast(), usage);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    //TODO: check for data vs allocated size!
    pub fn sub_buffer_data<D: Sized>(&self, offset: GLintptr, data: &[D]) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
            gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, offset, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for IndicesBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}


pub struct ShaderStorageBuffer {
    pub id: GLuint,
}


impl ShaderStorageBuffer {

    pub fn new() -> ShaderStorageBuffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        ShaderStorageBuffer{ id }
    }

    pub fn buffer_data<D: Sized>(&self, data: &[D], usage: GLenum) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
            gl::BufferData(gl::SHADER_STORAGE_BUFFER, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast(), usage);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    //TODO: check for data vs allocated size!
    pub fn sub_buffer_data<D: Sized>(&self, offset: GLintptr, data: &[D]) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
            gl::BufferSubData(gl::SHADER_STORAGE_BUFFER, offset, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast());
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    pub fn bind_buffer_base(&self, binding_point: GLuint) {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding_point, self.id);
        }
    }

    //TODO: bind_buffer_range()

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}

impl Drop for ShaderStorageBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}


pub struct UniformBuffer {
    pub id: GLuint,
}

impl UniformBuffer {

    pub fn new() -> UniformBuffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        UniformBuffer{ id }
    }

    pub fn buffer_data<D: Sized>(&self, data: &[D], usage: GLenum) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.id);
            gl::BufferData(gl::UNIFORM_BUFFER, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast(), usage);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    //TODO: check for data vs allocated size!
    pub fn sub_buffer_data<D: Sized>(&self, offset: GLintptr, data: &[D]) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.id);
            gl::BufferSubData(gl::UNIFORM_BUFFER, offset, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast());
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    pub fn bind_buffer_base(&self, binding_point: GLuint) {
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, self.id);
        }
    }

    //TODO: bind_buffer_range()

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}

impl Drop for UniformBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}


pub struct VertexBuffer {
    pub id: GLuint,
}

impl VertexBuffer {

    pub fn new() -> VertexBuffer{
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        VertexBuffer{ id }
    }

    pub fn buffer_data<D: Sized>(&self, data: &[D], usage: GLenum) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast(), usage);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    //TODO: check for data vs allocated size!
    pub fn sub_buffer_data<D: Sized>(&self, offset: GLintptr, data: &[D]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferSubData(gl::ARRAY_BUFFER, offset, (std::mem::size_of::<D>() * data.len()) as GLsizeiptr, data.as_ptr().cast());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn set_attribute_pointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLint, pointer: GLint, divisor: GLuint) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::VertexAttribPointer(index, size, type_, gl::FALSE, stride, pointer as *const _);
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribDivisor(index, divisor);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}
