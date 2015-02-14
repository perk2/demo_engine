use std::io::File;
use std::io::fs::PathExtensions;
use std::mem;
use std::ptr;

use gles::types::*;
use gles::;

use shader::Program;

pub struct Scene{
  // TODO: How should the scene graph be desribed in rust?
  //       Could build an actual tree. For now will have one
  //       vector of programs and a program will have a vector of models.
  //       Will see how this goes with deeper layers of models.
  lol: u32
  //programs: Vec<Program>
}

enum State{init, parse_tree, parse_program, parse_model}

impl Scene{
  pub fn new(filename: &str) -> Scene {
    let contents = File::open(&Path::new(filename)).read_to_string().unwrap();
    let state: State = State::init;
    // Parse the scene graph and add nodes to the Scene.
    // Add state machine ENUM. Check states while parsing.
    let c_iter = contents.chars();
    let c: char;
    loop {
      // State machine:
      // Search for starting {
      // Search for program node
      // Parse model.
      // Parse program.
      // JSON parsing state machine
      c = c_iter.next().unwrap();
      match state {
        init => {
          if(c=="{") {
            state = State::parse_tree;
          }
        },
        parse_tree => {
          // Find type of the first node and select next state
          let mut node_type = "";
        }
        _    => println!("Error: Unknown state"),
      }
      println!("{}", c);
    }
    Scene{lol: 5u32}
  }
}

struct Model{
  // Vertex positions, color etc
  vertex_data: Vec<GLfloat>,
  vertex_indices: Vec<GLuint>,
  // TODO: These could maybe be optional?
  vbo_handle: GLuint,
  ebo_handle: GLuint
}

impl Model{
  fn new() -> Model {
    let vd: Vec<f32> = vec![0.0, 0.0, -0.5,
                            0.5, 0.0, -0.5,
                            0.5, 0.5, -0.5];
    let vi: Vec<u32> = vec![0, 1, 2];

    // Bind the buffers
    let mut ebo_handle = 0;
    let mut vbo_handle = 0;
    unsafe {
      // Create VBO
      gles::GenBuffers(1, &mut vbo_handle);
      gles::BindBuffer(gles::ARRAY_BUFFER, vbo_handle);
      gles::BufferData(gles::ARRAY_BUFFER,
                       (vd.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(vd.as_ptr()),
                       gles::STATIC_DRAW);

      // Create EBO
      gles::GenBuffers(1, &mut ebo_handle);
      assert!(gles::GetError() == gles::NO_ERROR);
      gles::BindBuffer(gles::ELEMENT_ARRAY_BUFFER, ebo_handle);
      assert!(gles::GetError() == gles::NO_ERROR);
      gles::BufferData(gles::ELEMENT_ARRAY_BUFFER,
                       (vi.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                       mem::transmute(vi.as_ptr()),
                       gles::STATIC_DRAW);
      assert!(gles::GetError() == gles::NO_ERROR);
    }
    Model{vertex_data:vd, vertex_indices: vi,
          vbo_handle: vbo_handle, ebo_handle: ebo_handle}
  }
}
