/*
This file contains global constants such as colors
*/

// Window sizes
pub const GRID_BLOCK_BUFFER:usize = 8;
pub const GRID_BLOCK_SIZE_X:usize = 22 + GRID_BLOCK_BUFFER;  // block wide window with block buffer because of tetrimino::MinoShape::I 
pub const GRID_BLOCK_SIZE_Y:usize = 25 + GRID_BLOCK_BUFFER;  // block high window with block buffer because of tetrimino::MinoShape::I
pub const WIN_SIZE_X:f64  = (GRID_BLOCK_SIZE_X - GRID_BLOCK_BUFFER) as f64 * BLOCK_DIM;  //450.0;
pub const WIN_SIZE_Y:f64  = (GRID_BLOCK_SIZE_Y - GRID_BLOCK_BUFFER) as f64 * BLOCK_DIM;  //580.0;
pub const BORDER_THICKNESS:f64 = 5.0;
//Stuff we need for rendering text on the generated window
pub const FONT_SIZE:u32 = 12;
pub const FONT_NAME:&str = "LCD_SOLID.ttf";
//pub const FONT_PATH = Path::new("C:USERS/HULDA/APPDATA/MICROSOFT/WINDOWS");

// Tetris
pub const BLOCK_DIM:f64 = 20.0;             // Number of pixels in the width and height of a block (blocks are used to create tetriminos and this dimension is used for the window grid)
pub const BLOCK_BORDER_THICKNESS:f64 = 2.0;
pub const ZONE_BLOCK_HEIGHT: usize = 20;    // How many blocks the zone height is
pub const ZONE_BLOCK_WIDTH: usize = 10;     // How many blocks the zone width is
pub const TETRIS_ZONE_POS: [usize; 4] = [2,
                                      2,
                                      2 + ZONE_BLOCK_WIDTH,
                                      2 + ZONE_BLOCK_HEIGHT];            // The play zone position
pub const NEXT_ZONE_POS: [usize; 4] = [13,
                                       4,
                                       19,
                                       10];                              // The next tetrimino zone position
pub const FALL_START_POS: [usize; 2] = [(TETRIS_ZONE_POS[2] - TETRIS_ZONE_POS[0])/2, TETRIS_ZONE_POS[1]+2];


// Colours
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 0.8, 1.0];
pub const PURPLE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.368, 0.075, 1.0];
pub const LIGHTBLUE: [f32; 4] = [0.3, 0.3, 1.0, 1.0];  


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

