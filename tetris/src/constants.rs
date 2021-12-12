/*
This file contains global constants such as colors
*/

// Colours
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const PURPLE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
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

