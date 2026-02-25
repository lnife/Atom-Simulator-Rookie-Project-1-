use nalgebra_glm as glm;
use std::f32::consts::PI;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub unsafe fn new(vs_src: &CStr, fs_src: &CStr) -> Self {
        let vertex_shader = shader_from_source(vs_src, gl::VERTEX_SHADER);
        let fragment_shader = shader_from_source(fs_src, gl::FRAGMENT_SHADER);

        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex_shader);
        gl::AttachShader(id, fragment_shader);
        gl::LinkProgram(id);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Self { id }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_uniform_4f(&self, name: &CStr, v0: f32, v1: f32, v2: f32, v3: f32) {
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::Uniform4f(location, v0, v1, v2, v3);
    }

    pub unsafe fn set_uniform_mat4(&self, name: &CStr, mat: &glm::Mat4) {
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct VertexArray {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    vertex_count: i32,
}

impl VertexArray {
    pub unsafe fn new(vertices: &[f32]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let vertex_count = (vertices.len() / 3) as i32;

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<f32>()) as isize,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * mem::size_of::<f32>()) as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        Self {
            vao,
            vbo,
            vertex_count,
        }
    }

    pub fn vertex_count(&self) -> i32 {
        self.vertex_count
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.vao);
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

pub fn generate_sphere(radius: f32, sectors: i32, stacks: i32) -> Vec<f32> {
    let mut vertices = Vec::new();
    let sector_step = 2.0 * PI / sectors as f32;
    let stack_step = PI / stacks as f32;

    for i in 0..stacks {
        let stack_angle1 = PI / 2.0 - (i as f32 * stack_step);
        let stack_angle2 = PI / 2.0 - ((i + 1) as f32 * stack_step);

        for j in 0..sectors {
            let sector_angle1 = j as f32 * sector_step;
            let sector_angle2 = (j + 1) as f32 * sector_step;

            // Vertices of the quad
            let v1 = glm::vec3(
                radius * stack_angle1.cos() * sector_angle1.cos(),
                radius * stack_angle1.cos() * sector_angle1.sin(),
                radius * stack_angle1.sin(),
            );
            let v2 = glm::vec3(
                radius * stack_angle1.cos() * sector_angle2.cos(),
                radius * stack_angle1.cos() * sector_angle2.sin(),
                radius * stack_angle1.sin(),
            );
            let v3 = glm::vec3(
                radius * stack_angle2.cos() * sector_angle1.cos(),
                radius * stack_angle2.cos() * sector_angle1.sin(),
                radius * stack_angle2.sin(),
            );
            let v4 = glm::vec3(
                radius * stack_angle2.cos() * sector_angle2.cos(),
                radius * stack_angle2.cos() * sector_angle2.sin(),
                radius * stack_angle2.sin(),
            );

            // Triangle 1
            vertices.extend_from_slice(&[v1.x, v1.y, v1.z]);
            vertices.extend_from_slice(&[v3.x, v3.y, v3.z]);
            vertices.extend_from_slice(&[v2.x, v2.y, v2.z]);

            // Triangle 2
            vertices.extend_from_slice(&[v2.x, v2.y, v2.z]);
            vertices.extend_from_slice(&[v3.x, v3.y, v3.z]);
            vertices.extend_from_slice(&[v4.x, v4.y, v4.z]);
        }
    }
    vertices
}

// Helper function to compile a shader from source
unsafe fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> gl::types::GLuint {
    let id = gl::CreateShader(kind);
    gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
    gl::CompileShader(id);

    let mut success: gl::types::GLint = 1;
    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        let mut info_log = Vec::with_capacity(len as usize);
        info_log.set_len((len as usize) - 1);
        gl::GetShaderInfoLog(id, len, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
        panic!("Shader compilation error: {}", String::from_utf8_lossy(&info_log));
    }
    id
}
