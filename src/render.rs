use nalgebra_glm as glm; // math because trig is pain
use std::f32::consts::PI; // pi because sphere
use std::ffi::CStr; // C strings for opengl
use std::mem; // size_of for buffer sizes
use std::os::raw::c_void; // raw pointer casting
use std::ptr; // null pointers etc

pub struct ShaderProgram {
    // wrapper around opengl shader program
    // holds program id
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub unsafe fn new(vs_src: &CStr, fs_src: &CStr) -> Self {
        // compile vertex and fragment shaders
        // attach them
        // link them
        // delete intermediates
        // classic opengl ritual

        let vertex_shader = shader_from_source(vs_src, gl::VERTEX_SHADER);
        let fragment_shader = shader_from_source(fs_src, gl::FRAGMENT_SHADER);

        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex_shader);
        gl::AttachShader(id, fragment_shader);
        gl::LinkProgram(id);

        // once linked, shaders can be deleted
        // program keeps compiled version internally
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Self { id }
    }

    pub unsafe fn use_program(&self) {
        // tells opengl "use this shader now"
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_uniform_4f(&self, name: &CStr, v0: f32, v1: f32, v2: f32, v3: f32) {
        // set vec4 uniform (color mostly)
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::Uniform4f(location, v0, v1, v2, v3);
    }

    pub unsafe fn set_uniform_mat4(&self, name: &CStr, mat: &glm::Mat4) {
        // send 4x4 matrix to shader
        // used for model/view/projection
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        // rust cleanup hook
        // delete program when struct goes out of scope
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct VertexArray {
    // wrapper for vao + vbo
    // because raw opengl calls everywhere is messy
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    vertex_count: i32,
}

impl VertexArray {
    pub unsafe fn new(vertices: &[f32]) -> Self {
        // create vertex array and buffer
        // upload vertex data to gpu

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

        // describe vertex layout to opengl
        // attribute 0 = position (vec3)
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * mem::size_of::<f32>()) as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // unbind to avoid accidental modification
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
        // bind vao before drawing
        gl::BindVertexArray(self.vao);
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        // cleanup gpu memory
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

pub fn generate_sphere(radius: f32, sectors: i32, stacks: i32) -> Vec<f32> {
    // manual sphere tessellation
    // generates triangles from lat-long grid
    // no indices, fully expanded triangle list

    let mut vertices = Vec::new();
    let sector_step = 2.0 * PI / sectors as f32;
    let stack_step = PI / stacks as f32;

    for i in 0..stacks {
        let stack_angle1 = PI / 2.0 - (i as f32 * stack_step);
        let stack_angle2 = PI / 2.0 - ((i + 1) as f32 * stack_step);

        for j in 0..sectors {
            let sector_angle1 = j as f32 * sector_step;
            let sector_angle2 = (j + 1) as f32 * sector_step;

            // quad on sphere surface
            // split into two triangles

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

            // triangle 1
            vertices.extend_from_slice(&[v1.x, v1.y, v1.z]);
            vertices.extend_from_slice(&[v3.x, v3.y, v3.z]);
            vertices.extend_from_slice(&[v2.x, v2.y, v2.z]);

            // triangle 2
            vertices.extend_from_slice(&[v2.x, v2.y, v2.z]);
            vertices.extend_from_slice(&[v3.x, v3.y, v3.z]);
            vertices.extend_from_slice(&[v4.x, v4.y, v4.z]);
        }
    }

    vertices
}

// compile shader from source
// this is where errors scream loudly
unsafe fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> gl::types::GLuint {
    let id = gl::CreateShader(kind);
    gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
    gl::CompileShader(id);

    let mut success: gl::types::GLint = 1;
    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

    if success == 0 {
        // if compilation fails, grab error log and panic
        let mut len: gl::types::GLint = 0;
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

        let mut info_log = Vec::with_capacity(len as usize);
        info_log.set_len((len as usize) - 1);

        gl::GetShaderInfoLog(
            id,
            len,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut gl::types::GLchar,
        );

        panic!(
            "shader compilation error: {}",
            String::from_utf8_lossy(&info_log)
        );
    }

    id
}
