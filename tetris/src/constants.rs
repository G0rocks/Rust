/*
This file contains global constants such as colors
*/
use std::path::Path;        // Used to find the font file
use graphics::types::FontSize; // To be able to set the font size

// Window sizes
pub const WIN_SIZE_X:f64  = 500.0;  //450.0;
pub const WIN_SIZE_Y:f64  = 600.0;  //580.0;
pub const BORDER_THICKNESS:f64 = 5.0;
//Stuff we need for rendering text on the generated window
pub const FONT_SIZE:FontSize = 12;
pub const FONT_NAME:&str = "LCD_SOLID.ttf";
//pub const FONT_PATH = Path::new("C:USERS/HULDA/APPDATA/MICROSOFT/WINDOWS");

// Tetris
pub const BLOCK_WIDTH:u32 = 25;                                // Number of pixels in the width of a square (squares are used to create tetriminos)
pub const BLOCK_HEIGHT:u32 = BLOCK_WIDTH;                               // Number of pixels in the height of a square (squares are used to create tetriminos)

// Colours
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 0.8, 1.0];
pub const PURPLE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.368, 0.075, 1.0];
pub const LIGHTBLUE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];  


// Stigagjöf (e. point system)
/*
Fyrir 1 línu - 100 stig
Fyrir 2 línur - 250 stig
Fyrir 3 línur - 400 stig
Fyrir 4 línur (einnig þekkt sem Tetris) - 800 stig
Fyrir að fá Tetris tvisvar í röð - 1200 stig (fást fyrir seinna Tetris-ið)
Þegar leikmaður leggur tetrimino niður með því að hard drop-pa
10 stig fyrir hvern reit sem tetrimino-inn féll niður um í hard dropp-inu
*/
pub const POINTS_1_LINE:u8 = 100;
pub const POINTS_2_LINES:u8 = 250;
pub const POINTS_3_LINES:u16 = 400;
pub const POINTS_TETRIS:u16 = 800;
pub const POINTS_DOUBLE_TETRIS:u16 = 1200;
pub const POINTS_SQUARE_HARD_DROP:u8 = 10;

