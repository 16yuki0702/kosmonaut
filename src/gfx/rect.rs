use gl::program::Program;
use gl::shader::{Shader, ShaderKind};
use gl::types::GLint;
use gl::vao::VertexArrayObject;
use gl::vbo::VertexBufferObject;
use gl::Gl;
use std::ffi::CString;

/// Uses given OpenGL instance to paint arbitrary rectangles.
pub struct RectPainter {
    /// The OpenGL program that will be used to paint rectangles.
    program: Program,
    /// The VAO to use to draw rectangles.
    vao: VertexArrayObject,
    /// An instance of OpenGL.
    gl: Gl,
}

impl RectPainter {
    pub fn new(gl: &Gl) -> Result<RectPainter, String> {
        let vbo = VertexBufferObject::new(gl);
        let config_rect_vao = |gl: &Gl| {
            unsafe {
                gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
                gl.VertexAttribPointer(
                    0,         // index of the generic vertex attribute ("layout (location = 0)")
                    3,         // the number of components per generic vertex attribute
                    gl::FLOAT, // data type
                    gl::FALSE, // normalized (int-to-float conversion)
                    (3 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                    std::ptr::null(),                          // offset of the first component
                );
            }
        };
        let vao = unsafe { VertexArrayObject::new(vbo, config_rect_vao, gl) };

        Ok(RectPainter {
            program: build_triangle_program(gl)?,
            gl: gl.clone(),
            vao,
        })
    }

    //    pub fn paint(&self, rect: Rect, rgba: RGBA) {
    pub fn paint(&mut self, data: &[f32]) {
        self.program.use_globally();
        self.vao.store_vertex_data(&data[..]);
        unsafe {
            self.gl.BindVertexArray(self.vao.name());
            self.gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of vertices to be rendered
            );
            self.gl.BindVertexArray(0);
        }
    }
}

fn build_triangle_program(gl: &Gl) -> Result<Program, String> {
    let vertex_shader = Shader::from_source(
        &CString::new(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shader_src/triangle.vert"
        )))
        .expect("could not create cstring for triangle"),
        ShaderKind::Vertex,
        gl,
    )?;
    let fragment_shader = Shader::from_source(
        &CString::new(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shader_src/triangle.frag"
        )))
        .expect("could not create cstring for triangle"),
        ShaderKind::Fragment,
        gl,
    )?;
    let program = Program::from_shaders(&[vertex_shader, fragment_shader], &gl)?;
    Ok(program)
}
