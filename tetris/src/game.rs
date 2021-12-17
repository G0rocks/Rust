/*
Here the core game resides
Contains the struct and implementation of the game itself
*/

// Declare crates
use piston::input::*;       // Used to get the render input from user
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
    let tetrion_inner_square = graphics::rectangle::rectangle_by_corners(constants::WIN_SIZE_X*0.05, constants::WIN_SIZE_Y*0.05, self.zone_width as f64, self.zone_height as f64);
    let tetrion_outer_square = graphics::rectangle::rectangle_by_corners(constants::WIN_SIZE_X*0.05 - constants::BORDER_THICKNESS, constants::WIN_SIZE_Y*0.05 - constants::BORDER_THICKNESS, (self.zone_width as f64) + constants::BORDER_THICKNESS, (self.zone_height as f64) + constants::BORDER_THICKNESS);

    // Render score, level and such
    let next_mino_inner_square = graphics::rectangle::rectangle_by_corners(constants::WIN_SIZE_X*0.6, constants::WIN_SIZE_Y*0.1, constants::WIN_SIZE_X*0.8, constants::WIN_SIZE_Y*0.3);
    let next_mino_outer_square = graphics::rectangle::rectangle_by_corners(constants::WIN_SIZE_X*0.6 - constants::BORDER_THICKNESS, constants::WIN_SIZE_Y*0.1 - constants::BORDER_THICKNESS, constants::WIN_SIZE_X*0.8 + constants::BORDER_THICKNESS, constants::WIN_SIZE_Y*0.3 + constants::BORDER_THICKNESS);
    //let score_text = "Score: ";
    //let score_render = graphics::text(constants::WHITE, constants::FONT_SIZE, &score_text, cache: &mut C, transform: math::Matrix2d, g: &mut G)

    // Implement the renders so far
    self.gl.draw(arg.viewport(), |c,gl|{
      let transform = c.transform;

      graphics::rectangle(constants::WHITE, tetrion_outer_square, transform, gl);
      graphics::rectangle(constants::BLACK, tetrion_inner_square, transform, gl);
      graphics::rectangle(constants::WHITE, next_mino_outer_square, transform, gl);
      graphics::rectangle(constants::BLACK, next_mino_inner_square, transform, gl);
    });


    // Teiknum leiðbeiningar fyrir neðan tetris zone-ið

    // Sínum á hvaða borði (e. level) leikmaðurinn er

    // Sínum fjölda lína sem leikmaðurinn hefur tekist að láta hverfa

  
    // Render each tetrimino (the first one is the one on hold and the second one is the next tetrimino and the third one is the active tetrimino)
    for i in 0..self.minos.len() {
      self.minos[i].render(arg); //arg);
    }
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
    // Firstly if nowe need a new tetrimino, add a new one
    if 2 > self.minos.len() {
      self.minos.push(tetrimino::Mino::new())
    }
    //Here we need to move all the tetriminos like they're supposed to move or 

    // Látum tetrimino-ana detta, einn í einu, rólega niður

    // Snúum tetrimino-inum þegar leikmaðurinn ýtir á upp örina
    // Færum tetrimino-inn til hægri/vinstri þegar leikmaðurinn ýtir á hægri/vinstri örvatakkana

    // Látum tetrimino-inn detta hratt niður á meðan leikmaðurinn heldur niðri niður örvatakkanum

    // Setja þá alla leið niður ef leikmaður ýtir á bilslánna - Þekkt sem "Hard drop"

    // Þegar tetriminoarnir mynda heila, óbrotna, lárétta línu yfir allan leikskjáinn þá hverfa þeira og leikmaðurinn fær
    // stig í samræmi við hversu margar línur hurfu samtímis.
    // Láta alla kubba fyrir ofan línuna detta niður þegar línan/línurnar hverfa
    // Færum leikmann á næsta borð (e. level) þegar leikmaður nær ákveðið mörgum stigum. Hraðinn eykst eftir því sem þú kemst á hærra borð

    // Láta tetrimino-a stoppa þegar þeir rekast á annan tetrimino fyrir neðan sig
    // Setja næsta tetrimino af stað þegar núverandi tetrimino stoppar
    // Láta leikinn enda þegar tetrimino er stoppar og er fyrir ofan toppinn á skjánum
    // Þegar leikurinn endar --> Sýna "GAME OVER" og stig leikmanns
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
  
