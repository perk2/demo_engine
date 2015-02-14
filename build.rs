// This file is used to build OpenGLES bindings

extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::io::File;

fn main() {
  let dest = Path::new(os::getenv("OUT_DIR").unwrap());

  let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

  // Generate bindings for OpenGL ES v3.1 and store in file
  gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                  gl_generator::registry::Ns::Gles2,
                                  khronos_api::GL_XML,
                                  vec![],
                                  "3.1", "core", &mut file).unwrap();
}
