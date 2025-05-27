mod gl_glfw;
extern crate glm;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

const G_VERTEX_BUFFER_DATA: [gl::types::GLfloat; 12] = [
    0.5,  0.5, 0.0,  // top right
    0.5, -0.5, 0.0,  // bottom right
   -0.5, -0.5, 0.0,  // bottom left
   -0.5,  0.5, 0.0   // top left 
];
const INDICIES: [u32; 6] = [  // note that we start from 0!
   0, 1, 3,   // first triangle
   1, 2, 3    // second triangle
];  

const VERTEX_SHADER_SRC: &str = "
#version 330 core

layout (location = 0) in vec3 position;

uniform float my_trans;

const float our_trans = 0.5;

void main() {
    gl_Position = vec4(our_trans, our_trans, our_trans, 1.0) * vec4(position, 1.0);
    //gl_Position = vec4(our_trans*position, 1.0);
}
";

const FRAGMENT_SHADER_SRC: &str = "
# version 330 core

uniform float my_trans;

out vec4 colour;

void main() {
    // return the colour blue
    colour = vec4(0.0, 0.0, 1.0, 1.0);
}
";

fn mat4_to_float_array(mat:glm::Matrix4<f32>) -> [f32;16]
{
    let trans_array: [f32; 16] = [
        mat.c0.x,
        mat.c0.y,
        mat.c0.z,
        mat.c0.w,
        mat.c1.x,
        mat.c1.y,
        mat.c1.z,
        mat.c1.w,
        mat.c2.x,
        mat.c2.y,
        mat.c2.z,
        mat.c2.w,
        mat.c3.x,
        mat.c3.y,
        mat.c3.z,
        mat.c3.w,
        ];
    println!("\nmat.c0.x = {}, mat.c1.y = {}\n", mat.c0.x, mat.c1.y);
    return trans_array;
}

fn main() {
    let (mut glfw, mut window, mut events) = gl_glfw::initialize_critical(
                                            300,
                                            300,
                                            "Hello this is window", 
                                            "Failed to create GLFW window.");
    let (obj_vertex_array_id, obj_buffer_id, obj_element_buffer_id) = 
        gl_glfw::create_obj(&G_VERTEX_BUFFER_DATA, &INDICIES);    
    let shader_program = gl_glfw::compile_shader_program(
                            VERTEX_SHADER_SRC, 
                            FRAGMENT_SHADER_SRC);

    let ones: [gl::types::GLfloat; 16] = [1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0];
    //let mut trans: glm::Mat4  = glm::mat4(
    //    1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0);
    //trans = glm::ext::rotate(&trans, glm::radians(90.0), glm::vec3(0.0, 0.0, 1.0));
    //trans = glm::ext::scale(&trans, glm::vec3(0.5, 0.5, 0.5));  

    let name = CString::new("my_trans").expect("CString::new failed");
    let transform_loc = unsafe { gl::GetUniformLocation(shader_program, name.as_ptr()) };
    println!("\n{}\n", transform_loc);

    unsafe { gl::UseProgram(shader_program); };
    //let array = mat4_to_float_array(trans);
    //unsafe { gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, ones.as_ptr()) };
    let val: gl::types::GLfloat = 1.9567;
    unsafe { gl::Uniform1f(transform_loc, val) };
    let mut help: gl::types::GLfloat = 1.0;
    unsafe { gl::GetUniformfv(shader_program, transform_loc, &mut help) };
    println!("\nmy_trans = {}\n", help);

    while !window.should_close() {
        (glfw, window, events) = gl_glfw::tick_beginning(glfw, window, events);
        // 1st attribute buffer : vertices
        
        unsafe { gl::UseProgram(shader_program); };
        unsafe { gl::Uniform1f(transform_loc, val) }
        unsafe { gl::BindVertexArray(obj_vertex_array_id) };
        unsafe { gl::Uniform1f(transform_loc, val) }

        unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _) };

    }

    // Free OpenGL memory
    gl_glfw::delete_obj(obj_vertex_array_id, obj_buffer_id, obj_element_buffer_id); 
    gl_glfw::delete_shader_program(shader_program);
}
