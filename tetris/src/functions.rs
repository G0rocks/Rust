/*
  This file contains functions that will be called in the main file
  These functions have descriptive names and the goal is to simplify the main.rs file
  as well as to simplify the coding
*/

//Function which returns false if the game is over and true if the game is still on
pub fn game_is_on(top_line:u8, tetris_zone_height:u8) -> bool {
  //check if we have topped out
  if top_line < tetris_zone_height {
    return true;
  }
  else {
  return false;
  }
}
