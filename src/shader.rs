use std::io::File;
use std::io::fs::PathExtensions;
use std::ptr;
use std::ffi::CString;
use gles::types::*;
// TODO: Everything is public :(
use gles::;
pub struct Shader {
  shader_string: String,
  shader_type: GLenum,
  id: GLuint
}

impl Shader {
  // Read the shader_string from the given filename and store in Shader
  pub fn new(filename: &str, shader_type: GLenum) -> Shader{
    let contents = File::open(&Path::new(filename)).read_to_end().unwrap();
    let s = String::from_utf8(contents).unwrap();
    let cs= CString::from_slice(s.as_slice().as_bytes()).as_ptr();
    let id;
    unsafe {
      id=gles::CreateShader(shader_type);
      gles::ShaderSource(id, 1i32, &cs, ptr::null());
      gles::CompileShader(id);
      // Check for errors
      let mut status: GLint = 0;
      gles::GetShaderiv(id, gles::COMPILE_STATUS, &mut status);
      match status as GLubyte{
        // TODO: Should print debug/panic here
        gles::FALSE => println!("Shader compilation failed."),
        gles::TRUE  => println!("Shader compilation successful."),
        _ => println!("Shader compilation returned unknown value.")
      }
    }
    Shader{shader_type: shader_type, shader_string: s, id: id}
  }

}

pub struct Program {
  // TODO: Do I actually need the shader_ids anywhere?
  vs: Shader,
  fs: Shader,
  pub id: GLuint
}

impl Program {
  pub fn new(vs: Shader, fs: Shader) -> Program{
    let id;
    unsafe {
      id = gles::CreateProgram();
      gles::AttachShader(id, vs.id);
      gles::AttachShader(id, fs.id);
      gles::LinkProgram(id);
      gles::UseProgram(id);
      assert!(gles::GetError() == gles::NO_ERROR);
    }
    Program{vs: vs, fs: fs, id: id}
  }
}

