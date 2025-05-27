extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};

pub fn initialize_critical(
    size_x: u32,
    size_y: u32,
    success_msg: &str, 
    error_msg: &str) -> (
        glfw::Glfw, 
        glfw::Window, 
        std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>)
{
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = 
        glfw.create_window(size_x, size_y, success_msg, glfw::WindowMode::Windowed)
        .expect(error_msg);

    window.set_key_polling(true);
    window.make_current();

    // the supplied function must be of the type:
    // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
    // `window` is a glfw::Window
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // loading a specific function pointer
    gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);
    return (glfw, window, events);
}

pub fn tick_beginning(
    mut glfw: glfw::Glfw, 
    mut window: glfw::Window, 
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>)-> (
        glfw::Glfw, 
        glfw::Window, 
        std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>)
{
    window.swap_buffers();
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) 
    {
        handle_window_event(&mut window, event);
    }

    unsafe { gl::ClearColor(1.0, 0.0, 1.0, 0.0) }; // Magenta
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
    return (glfw, window, events);
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) 
{
    match event 
    {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

pub fn compile_shader_program(
    vertex_shader_src: &str, 
    fragment_shader_src: &str) -> gl::types::GLuint
{
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    // Set the source
    unsafe {
        gl::ShaderSource(
            vertex_shader,                      // shader id
            1,                                  // number of shaders
            &vertex_shader_src.as_ptr().cast(), // the shader source
            &(vertex_shader_src.len() as i32)   // the length of the source
        )
    };

    // Compile the shader
    unsafe { gl::CompileShader(vertex_shader) };

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    // Set the source
    unsafe {
        gl::ShaderSource(
            fragment_shader,                      // shader id
            1,                                  // number of shaders
            &fragment_shader_src.as_ptr().cast(), // the shader source
            &(fragment_shader_src.len() as i32)   // the length of the source
        )
    };

    // Compile the shader
    unsafe { gl::CompileShader(fragment_shader) };

    // Create an empty program
    let shader_program = unsafe { gl::CreateProgram() };

    // Attach the vertex and fragment shaders
    // to the program
    unsafe { gl::AttachShader(shader_program, vertex_shader) };
    unsafe { gl::AttachShader(shader_program, fragment_shader) };

    // Link the program
    unsafe { gl::LinkProgram(shader_program) };

    return shader_program;
}

pub fn delete_shader_program(
    shader_program_id: gl::types::GLuint)
{
    unsafe { gl::DeleteProgram(shader_program_id) };
}

pub fn create_obj(
    vertex_buffer_data: &[gl::types::GLfloat],
    element_buffer_data: &[u32]) -> (
        gl::types::GLuint,
        gl::types::GLuint,
        gl::types::GLuint)
{
    let mut vertex_array_id: gl::types::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vertex_array_id) };
    unsafe { gl::BindVertexArray(vertex_array_id) };

    // This will identify our vertex buffer
    let mut vertex_buffer_id: gl::types::GLuint = 0;
    // Generate 1 buffer, put the resulting identifier in vertex_buffer_id
    unsafe { gl::GenBuffers(1, &mut vertex_buffer_id) };
    // The following commands will talk about our 'vertex_buffer' buffer
    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id) };
    // Give our vertices to OpenGL.
    unsafe 
    { 
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            std::mem::size_of_val(vertex_buffer_data) as std::primitive::isize, 
            vertex_buffer_data.as_ptr().cast(), 
            gl::STATIC_DRAW
        ) 
    };

    // This will identify our vertex buffer
    let mut element_buffer_id: gl::types::GLuint = 0;
    // Generate 1 buffer, put the resulting identifier in vertex_buffer_id
    unsafe { gl::GenBuffers(1, &mut element_buffer_id) };
    // The following commands will talk about our 'vertex_buffer' buffer
    unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_id) };
    // Give our vertices to OpenGL.
    unsafe 
    { 
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, 
            std::mem::size_of_val(element_buffer_data) as std::primitive::isize, 
            element_buffer_data.as_ptr().cast(), 
            gl::STATIC_DRAW
        ) 
    };

    unsafe {
            gl::VertexAttribPointer(
                0,                  // attribute 0. No particular reason for 0, but must match the layout in the shader.
                3,                  // size
                gl::FLOAT,           // type
                gl::FALSE,           // normalized?
                0,                  // stride
                0 as *const _             // array buffer offset
            );
        }
    
    unsafe { gl::EnableVertexAttribArray(0) };

    return (vertex_array_id, vertex_buffer_id, element_buffer_id);
}

pub fn delete_obj(
    vertex_array_id: gl::types::GLuint,
    vertex_buffer_id: gl::types::GLuint,
    indicies_buffer_id: gl::types::GLuint)
{
    unsafe { gl::DeleteVertexArrays(1, &vertex_array_id) };
    unsafe { gl::DeleteBuffers(1, &vertex_buffer_id) };
    unsafe { gl::DeleteBuffers(1, &indicies_buffer_id) };
}
