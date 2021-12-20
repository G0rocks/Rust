/*
Here the core game resides
Contains the struct and implementation of the game itself
*/

// Declare crates
use piston::input::*;       // Used to get the render input from user
use crate::tetrimino;       // Tetriminos are used in the game
use crate::constants;       // For the colours
use colored::*;           // For coloring the terminal output

// Mayhaps not used?
//use piston_window;        // Used for rendering text
//use find_folder;          // For getting the font

// Dependencies
use opengl_graphics::GlGraphics;    //{GlGraphics, OpenGL};

/////////////////////////////////////////////////////////////////////////

pub struct Game {
  pub gl: GlGraphics,
  pub minos: Vec<tetrimino::Mino>,
  pub updates_per_second: u64,
  pub fall_counter: u64,
  pub fall_count_max: u64,
  pub audio_on: bool,
  pub game_over: bool,
  pub zone_pos: [usize; 4],   // [x1, y1, x2, y2] specifies the coordinates of the two corners of the play zone rectangle in grid coordinates
  pub next_pos: [usize; 4],   // [x1, y1, x2, y2] specifies the coordinates of the two corners of the next tetrimino rectangle in grid coordinates
  pub game_zone: [[i32; constants::GRID_BLOCK_SIZE_X]; constants::GRID_BLOCK_SIZE_Y],    // [x][y] position in the grid of the game zone
}


// Run application
impl Game {
  pub fn render(&mut self, arg: &RenderArgs) {
    // Render background
    self.gl.draw(arg.viewport(), |_c, gl| {
      graphics::clear(constants::BLACK,gl);
    });

    // Render outlines of tetrion
    let tetrion_inner_square = graphics::rectangle::rectangle_by_corners((self.zone_pos[0] as f64)*constants::BLOCK_DIM,
                                                                                 (self.zone_pos[1] as f64)*constants::BLOCK_DIM,
                                                                                 (self.zone_pos[2] as f64)*constants::BLOCK_DIM,
                                                                                 (self.zone_pos[3] as f64)*constants::BLOCK_DIM);  // self.zone_width as f64, self.zone_height as f64);
    let tetrion_outer_square = graphics::rectangle::rectangle_by_corners((self.zone_pos[0] as f64)*constants::BLOCK_DIM - constants::BORDER_THICKNESS,
                                                                                 (self.zone_pos[1] as f64)*constants::BLOCK_DIM - constants::BORDER_THICKNESS,
                                                                                 (self.zone_pos[2] as f64)*constants::BLOCK_DIM + constants::BORDER_THICKNESS,
                                                                                 (self.zone_pos[3] as f64)*constants::BLOCK_DIM + constants::BORDER_THICKNESS);

    // Next mino square
    let next_mino_inner_square = graphics::rectangle::rectangle_by_corners((self.next_pos[0] as f64)*constants::BLOCK_DIM,
                                                                            (self.next_pos[1] as f64)*constants::BLOCK_DIM,
                                                                            (self.next_pos[2] as f64)*constants::BLOCK_DIM,
                                                                            (self.next_pos[3] as f64)*constants::BLOCK_DIM);
    let next_mino_outer_square = graphics::rectangle::rectangle_by_corners((self.next_pos[0] as f64)*constants::BLOCK_DIM - constants::BORDER_THICKNESS,
                                                                            (self.next_pos[1] as f64)*constants::BLOCK_DIM - constants::BORDER_THICKNESS,
                                                                            (self.next_pos[2] as f64)*constants::BLOCK_DIM + constants::BORDER_THICKNESS,
                                                                            (self.next_pos[3] as f64)*constants::BLOCK_DIM + constants::BORDER_THICKNESS);

    // Render score, level and such
    //let score_text = "Score: ";

    // Implement the renders so far
    self.gl.draw(arg.viewport(), |c,gl|{
      let transform = c.transform;

      graphics::rectangle(constants::WHITE, tetrion_outer_square, transform, gl);
      graphics::rectangle(constants::BLACK, tetrion_inner_square, transform, gl);
      graphics::rectangle(constants::WHITE, next_mino_outer_square, transform, gl);
      graphics::rectangle(constants::BLACK, next_mino_inner_square, transform, gl);
      //piston_window::text(constants::WHITE, constants::FONT_SIZE, &score_text, c, transform, gl);
    });


    // Teiknum leiðbeiningar fyrir neðan tetris zone-ið

    // Sínum á hvaða borði (e. level) leikmaðurinn er
    //let assets = find_folder::Search::ParentsThenKids(3, 3)
        //.for_folder("assets").unwrap();
    //let ref font = assets.join(constants::FONT_NAME);
    //let mut glyphs = Glyphs::new(font, factory).unwrap();
    //text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
      //"level",
      //&mut glyphs,
      //&c.draw_state,
     // transform, g
    //);

    // Sínum fjölda lína sem leikmaðurinn hefur tekist að láta hverfa

  
    // Render each tetrimino (the first one is the one on hold and the second one is the next tetrimino and the third one is the active tetrimino)
    for i in 0..self.minos.len() {
      self.minos[i].render(arg);
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
    //println!("Updating game, minos.len: {}", self.minos.len());
    let mut next_counter : u32 = 0;   // Counts the number of minos in the tetrimino::MinoState::Next state
    if self.fall_counter == 0 {
      // Loop through each tetrimino. Update each tetrimino, the state, rotation and position of the tetrimino
      for i in 0..self.minos.len() {
        self.minos[i].update(); // Start by updating the mino

        //println!("Mino {} pos: x: {}, y: {}", i, self.minos[i].pos_x, self.minos[i].pos_y);
        //println!("Mino.last_state is {:?}", self.minos[i].last_state);
        match self.minos[i].state {
          tetrimino::MinoState::Falling => {  // Tetrimino is falling
            // Check last state, if last state was Next then set_start_fall_pos
            match self.minos[i].last_state {
              tetrimino::MinoState::Next => {
                self.minos[i].set_fall_start_pos();

                // If there is a block in line 3 (counted from the top), start two blocks higher than normally
                for x in 0..self.game_zone[0].len() {
                  if self.game_zone[constants::TETRIS_ZONE_POS[1] + 4][x] != 0 && self.game_zone[constants::TETRIS_ZONE_POS[1] + 2][x] != -1 {
                    self.minos[i].pos_y = self.minos[i].pos_y - 2;
                  }
                }
              }
              _=> {
              // Do nothing
              }
            }
            // Check if mino can keep falling
            // Check if the mino is on top of another mino
            for mut block in 0..self.minos[i].blocks.len() {
              let mino_below_value = self.game_zone[self.minos[i].blocks[block].pos_x][self.minos[i].blocks[block].pos_y + 2];
              if mino_below_value != 0 && mino_below_value != (i as i32 + 1) {
                self.minos[i].set_state(tetrimino::MinoState::Still);
                block = self.minos[i].blocks.len();
              }
            }

            // Check if the mino is at the bottom
            if self.minos[i].pos_y >= (self.zone_pos[3] - 1) {
              self.minos[i].set_state(tetrimino::MinoState::Still);
            }

            // If mino can't keep falling, set state to still and perform game over check

            // If mino can keep falling, make it continue. Make no change to x position -> self.minos[i].pos_x = self.minos[i].pos_x;
            self.minos[i].pos_y = self.minos[i].pos_y + 1; // Move down 1 block
          },
          tetrimino::MinoState::Next => {
            next_counter = next_counter + 1;
            // Check if there is a previous mino, if not, set this mino to falling
            if self.minos.len() < 2 {
              self.minos[i].set_state(tetrimino::MinoState::Falling);
              self.minos[i].set_fall_start_pos();   // Set the fall starting position so that we won't have 2 minos in the "Next" window at the same time
            }
            else {
              // Check the state of the previous mino

              // If the state is falling, or hold, do nothing, if the state is next, delete this mino, if the state is still, change the state of this mino to falling
              match self.minos[i-1].state {
                tetrimino::MinoState::Still => {
                  // Change the mino to falling, set to fall start pos and add a new mino
                  self.minos[i].set_state(tetrimino::MinoState::Falling);
                  self.minos[i].set_fall_start_pos();
                },
                tetrimino::MinoState::Next => {
                  self.minos.remove(i);
                },
                _ => {
                  // Do nothing
                },
              }
            }
          },
          tetrimino::MinoState::Hold => {
            // Do nothing
          },
          tetrimino::MinoState::None => {
            // Do nothing
          },
          tetrimino::MinoState::Still => {
            // Perform game over check
            for block in 0..self.minos[i].blocks.len() {
              if self.minos[i].blocks[block].pos_y <= constants::TETRIS_ZONE_POS[1] {
                println!("{}", "GAME OVER!!!".red());
                self.game_over = true;
              }
            }
          }
        }

        // Update the game zone for this mino
        self.update_game_zone(self.minos[i].clone(), (i+1) as i32);

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

      // print game zone
      //self.print_game_zone();

      // Reinitialize game zone
      self.reinitialize_game_zone();

      // Add new mino if there is no mino with the tetrimino::MinoState::Next state
      if next_counter == 0 {
        self.minos.push(tetrimino::Mino::new());
      }
    }

    // Update the fall counter
    self.fall_counter = (self.fall_counter+1)%self.fall_count_max;
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

  // Reinitializes the game zone
  fn reinitialize_game_zone(&mut self) {
    let (zone_x_min, zone_x_max, zone_y_min, zone_y_max) = ((constants::TETRIS_ZONE_POS[0] as i32-1) as usize, constants::TETRIS_ZONE_POS[2], constants::TETRIS_ZONE_POS[1], constants::TETRIS_ZONE_POS[3]+1);
    let (next_zone_x_min, next_zone_x_max, next_zone_y_min, next_zone_y_max) = ((constants::NEXT_ZONE_POS[0] as i32-1) as usize, constants::NEXT_ZONE_POS[2], constants::NEXT_ZONE_POS[1], constants::NEXT_ZONE_POS[3]+1);
    self.game_zone = [[0; constants::GRID_BLOCK_SIZE_X]; constants::GRID_BLOCK_SIZE_Y];
    for x in 0..self.game_zone.len() {  //y coord
      for y in 0..self.game_zone[x].len() {  // x coord
        if ((x >= zone_x_min && x <= zone_x_max) && y == zone_y_min) || ((x >= zone_x_min && x <= zone_x_max) && y == zone_y_max) || ((y >= zone_y_min && y <= zone_y_max) && x == zone_x_min) || ((y >= zone_y_min && y <= zone_y_max) && x == zone_x_max) {
          self.game_zone[x][y] = -1;
        }
        if ((x >= next_zone_x_min && x <= next_zone_x_max) && y == next_zone_y_min) || ((x >= next_zone_x_min && x <= next_zone_x_max) && y == next_zone_y_max) || ((y >= next_zone_y_min && y <= next_zone_y_max) && x == next_zone_x_min) || ((y >= next_zone_y_min && y <= next_zone_y_max) && x == next_zone_x_max) {
          self.game_zone[x][y] = -1;
        }
      }
    }
  }

  // Updates the game zone by placing the game_zone_pos given in the game zone
  fn update_game_zone(&mut self, mino: tetrimino::Mino, index: i32) {
    //println!("Updating game zone for mino {}", index.to_string().on_blue());
    self.game_zone[mino.blocks[0].pos_x][mino.blocks[0].pos_y] = index; // Block 1
    self.game_zone[mino.blocks[1].pos_x][mino.blocks[1].pos_y] = index; // Block 2
    self.game_zone[mino.blocks[2].pos_x][mino.blocks[2].pos_y] = index; // Block 3
    self.game_zone[mino.blocks[3].pos_x][mino.blocks[3].pos_y] = index; // Block 4
  }

  // prints the game zone to the terminal
  fn print_game_zone(&self) {
    println!("Game_zone, size: {}, {}, zone:", self.game_zone[1].len(), self.game_zone.len());
    for x in 0..self.game_zone[0].len() {
      for y in 0..self.game_zone.len() {
        if self.game_zone[y][x] == -1 {
          print!("{} ", "#".green());
        }
        else {
          if self.game_zone[y][x] != 0 {
            print!("{} ", self.game_zone[y][x].to_string().red());
          }
          else {
            print!("{} ", self.game_zone[y][x]);
          }
        }
      }
      print!("\n");
    }
    println!("");
  }

}
  
