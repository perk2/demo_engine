extern crate glfw;
extern crate cgmath;
extern crate libc;

use glfw::{Action, Context, Key};
use gles::types::*;
use std::mem;
use std::ptr;

mod gles {
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}
pub mod shader;
pub mod scene;

fn main() {
  // Create window
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();


  glfw.window_hint(glfw::WindowHint::Resizable(true));
  let (mut window, events) = glfw.create_window(
      800, 600, "Hello", glfw::WindowMode::Windowed).expect(
          "Failed to create GLFW window.");
  // TODO: What does this do?

  window.set_sticky_keys(true);
  window.set_key_polling(true);

  window.make_current();

  // Load function pointers, can use gles functions after this.
  gles::load_with(|s| window.get_proc_address(s));
  println!("Loaded function pointers");

  // Test some basic shaders
  let test_scene = scene::Scene::new(
    "/home/per/code/demo_engine/assets/test_scene.json");

  //let vertex_shader = shader::Shader::new("/home/per/code/nomad/basic.vert",
  //                                        gles::VERTEX_SHADER);
  //let fragment_shader = shader::Shader::new("/home/per/code/nomad/basic.frag",
  //                                          gles::FRAGMENT_SHADER);
  //println!("Loaded shaders");
  //let program = shader::Program::new(vertex_shader, fragment_shader);
  //println!("Loaded program");

  //// new() binds the buffers TODO: Change
  //let triangle = Model::new();
  //println!("Create triangle");
  //let vpos_loc;
  //unsafe{
  //  vpos_loc = gles::GetAttribLocation(program.id, "vertex_position".as_ptr() as *const i8);
  //  gles::BindBuffer(gles::ARRAY_BUFFER, triangle.vbo_handle);
  //  gles::VertexAttribPointer(vpos_loc as u32, 3, gles::FLOAT, gles::FALSE,
  //                            0,ptr::null());
  //  gles::EnableVertexAttribArray(vpos_loc as u32);
  //  gles::BindBuffer(gles::ELEMENT_ARRAY_BUFFER, triangle.ebo_handle);
  //  assert!(gles::GetError() == gles::NO_ERROR);
  //}



  unsafe {gles::ClearColor(0.0f32, 1.0f32, 1.0f32, 1.0f32)};
  //// Frame loop
  //println!("About to start frame loop");
  while !window.should_close() {
    unsafe {
      gles::Clear(gles::COLOR_BUFFER_BIT);
    //  gles::DrawElements(gles::TRIANGLES, triangle.vertex_indices.len() as i32,
    //                     gles::UNSIGNED_INT, ptr::null());
    }
      window.swap_buffers();
    glfw.poll_events();
    for event in glfw::flush_messages(&events) {
      handle_window_event(&window, event);
    }


  }
}

fn handle_window_event(window: &glfw::Window,
                       (time, event): (f64, glfw::WindowEvent)) {
  match event {
    // TODO: Handle keyevents.
    _ => {}
  }
}

