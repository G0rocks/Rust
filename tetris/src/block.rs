// Used to keep track of the blocks used in each tetrimino
//**************************************************************

// External crates
extern crate opengl_graphics;

// Dependencies
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;       // Used to get the render input from user
use crate::constants;   // For the dimensions

//#[derive(Copy)]
pub struct Block {
  gl: GlGraphics,
  col: [f32; 4],
  dim: u32,
  pub pos_x: f64,
  pub pos_y: f64,
}

impl Block {
  pub fn new(color: [f32; 4], x_pos: f64, y_pos: f64) -> Block {
    return Block{gl: GlGraphics::new(OpenGL::V4_5), col: color, dim: constants::BLOCK_DIM as u32, pos_x: x_pos, pos_y: y_pos};
  }

  pub fn render(&mut self, arg: &RenderArgs) {
    let thick = constants::BLOCK_BORDER_THICKNESS;
    let square_border = graphics::rectangle::rectangle_by_corners(
      self.pos_x as f64,
      self.pos_y as f64,
      self.pos_x as f64 + (constants::BLOCK_DIM as f64),
      self.pos_y as f64 - (constants::BLOCK_DIM as f64));
    let square = graphics::rectangle::rectangle_by_corners(
      self.pos_x as f64 + thick,
      self.pos_y as f64 - thick,
      self.pos_x as f64 - thick + (constants::BLOCK_DIM as f64),
      self.pos_y as f64 + thick - (constants::BLOCK_DIM as f64));
  
      let color = self.col;
      let mut border_color: [f32; 4] = self.col;
      border_color[3] = border_color[3]/(8.0 as f32);

    // Implement the renders so far
    self.gl.draw(arg.viewport(), |c,gl|{
      let transform = c.transform;
      graphics::rectangle(border_color, square_border, transform, gl);
      graphics::rectangle(color, square, transform, gl);
    });


  }
}

impl Clone for Block {
  fn clone (&self) -> Block {
    let clone_block: Block = Block {
      gl: GlGraphics::new(OpenGL::V4_5),
      col: self.col,
      dim: self.dim,
      pos_x: self.pos_x,
      pos_y: self.pos_y,
    };
    return clone_block;
  }
}