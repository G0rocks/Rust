// Used to keep track of the blocks used in each tetrimino
//**************************************************************

// External crates
extern crate opengl_graphics;

// Dependencies
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;       // Used to get the render input from user
use crate::game;
use crate::constants;   // For the dimensions

pub struct Block {
  gl: GlGraphics,
  col: [f32; 4],
  dim: u32,
  pos_x: i32,
  pos_y: i32,
}

impl Block {
  pub fn new(color: [f32; 4], x_pos: i32, y_pos: i32) -> Block {
    return Block{gl: GlGraphics::new(OpenGL::V4_5), col: color, dim: constants::BLOCK_DIM, pos_x: x_pos, pos_y: y_pos};
  }

  pub fn render(&mut self, arg: &RenderArgs) {
    let square = graphics::rectangle::rectangle_by_corners(
      self.pos_x as f64,
      self.pos_y as f64,
      self.pos_x as f64 + (constants::BLOCK_DIM as f64),
      self.pos_y as f64 + (constants::BLOCK_DIM as f64));

      let color = self.col;

    // Implement the renders so far
    &self.gl.draw(arg.viewport(), |c,gl|{
      let transform = c.transform;
      graphics::rectangle(color, square, transform, gl);
    });


  }
}