/*
Here the core game resides
Contains the struct and implementation of the game itself
*/

// Declare crates
use piston::input::*;       // Used to get input from user
use crate::tetrimino;       // Tetriminos are used in the game
use crate::constants;       // For the colours

// Dependencies
use opengl_graphics::GlGraphics;    //{GlGraphics, OpenGL};


/////////////////////////////////////////////////////////////////////////

pub struct Game {
  pub gl: GlGraphics,
  pub minos: Vec<tetrimino::Mino>,
  pub updates_per_second: u64,
  pub audio_on: bool,
  pub game_over: bool,
  pub zone_width: u32,
  pub zone_height: u32,
}



// Run application
impl Game {
  pub fn render(&mut self, arg: &RenderArgs) {
    // Render background
    self.gl.draw(arg.viewport(), |_c, gl| {
      graphics::clear(constants::BLACK,gl);
    });

    // Render outlines of tetrion
    let inner_square = graphics::rectangle::rectangle_by_corners(constants::WIN_SIZE_X*0.05, constants::WIN_SIZE_Y*0.05, self.zone_width as f64, self.zone_height as f64);
    let outer_square = graphics::rectangle::rectangle_by_corners(constants::WIN_SIZE_X*0.05 - constants::BOARDER_THICKNESS, constants::WIN_SIZE_Y*0.05 - constants::BOARDER_THICKNESS, (self.zone_width as f64) + constants::BOARDER_THICKNESS, (self.zone_height as f64) + constants::BOARDER_THICKNESS);

    // Render score, level and such
    //let score_text = "Score: ";
    //let score_render = graphics::text(constants::WHITE, constants::FONT_SIZE, &score_text, cache: &mut C, transform: math::Matrix2d, g: &mut G)

    // Implement the renders so far
    self.gl.draw(arg.viewport(), |c,gl|{
      let transform = c.transform;

      graphics::rectangle(constants::WHITE, outer_square, transform, gl);
      graphics::rectangle(constants::BLACK, inner_square, transform, gl);
    });

  
    // Render each tetrimino (the first one is the one on hold and the second one is the next tetrimino and the third one is the active tetrimino)
    //for i in 0..self.minos.len() {
      //self.minos[i].render(); //arg);
    //}
  }
  
  //This is supposed to be an end game message:
  /*
  fn game_over_render(&mut self, arg: &RenderArgs) {
    self.gl.draw(arg.viewport(), |_c, gl| {
      graphics::clear(BLACK,gl);
    });
          
    let game_over_text = "GAME OVER";
    let mut character_cache_C = "";
        
    self.gl.draw(arg.viewport(), |c,gl|{
      let transform = c.transform;
      graphics::text(WHITE, 18, &game_over_text, &mut character_cache_C, transform: math::Matrix2d, g: &mut G);
    });
  }
    */
        
  pub fn update(&mut self) {
    //Here we need to move all the tetriminos like they're supposed to move or 
  }

  /*
  //The function that is called when a button is pressed, changing the direction of the snake
  fn pressed(&mut self, btn: &Button) {
    self.vector[0].dir = match btn {
      &Button::Keyboard(Key::Up)
        if self.vector[0].last_dir != Direction::Down => Direction::Up,
      &Button::Keyboard(Key::Down)
        if self.vector[0].last_dir != Direction::Up => Direction::Down,
      &Button::Keyboard(Key::Left)
        if self.vector[0].last_dir != Direction::Right => Direction::Left,
      &Button::Keyboard(Key::Right)
        if self.vector[0].last_dir != Direction::Left => Direction::Right,
      
      // If m or M button is pressed, mute/unmute music
      // event button 'm' is pressed
      // If mute, unmute, if unmuted, mute.
      &Button::Keyboard(Key::M)   // Mute / unmute
        if self.vector[0].last_dir != Direction::Down => Direction::Up,

        //We default to the last direction so if somebody accidentally presses a different button, nothing happens
        _ => self.vector[0].dir.clone()
    }
  }
  */
}
  
