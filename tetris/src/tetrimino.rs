/*
  This file contains the structs for each of the tetriminos that will be called in the main file
  These structs have descriptive names and the goal is to simplify the main.rs file
  as well as to simplify the coding.
  All of them will have some methods that can act on them listed here:
    - rotate_clockwise()
    - rotate_counter_clockwise()
    - hard_drop()
    - 

// Útbúum tetrimino-a klasa - Þeir eru 7 - Munum eftir að gefa þeim allar þær skipanir sem þeir þurfa að kunna
//
//Define each tetrimino shape
//Make a method for moving them down 1 step

//Make a method for rotating them clockwise and counterclockwise


    # Tetrimino 1 - I
class tetrimino_I:
    # Þessi aðferð (e. method eða e. function) leyfir okkur að bæta við gildum þegar við smíðum (e. construct) nýtt eintak af klasanum
    def __init__(self, x = 0, y = 0, color = [0,0,200]) :
        self.x = x; self.y = y; self.color = color;

"""
    def draw(self, color):
        glBegin(GL_QUADS)
        glVertex2f(0.2*gluggi.width, 0.15*gluggi.height)
        glVertex2f(0.3*gluggi.width, 0.25*gluggi.height)
        glVertex2f(0.4*gluggi.width, 0.35*gluggi.height)
        glVertex2f(0.5*gluggi.width, 0.45*gluggi.height)
        glEnd()
    # Tetrimino 2 - O
    # Tetrimino 3 - T
    # Tetrimino 4 - S
    # Tetrimino 5 - Z
    # Tetrimino 6 - J
    # Tetrimino 7 - L

"""
*/

// External crates
extern crate opengl_graphics;

// Dependencies
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;       // Used to get the render input from user
use rand::{distributions::{Distribution, Standard},Rng,};
use crate::{block, constants};


// Defines the tetrimino structure
pub struct Mino {
    gl: GlGraphics,
    shape: MinoShape,
    color: [f32; 4],
    blocks: [block::Block; 4],
    pub pos_x: f64,
    pub pos_y: f64,
    block_offset: [[f64;2]; 4],
    pub state: MinoState,
    pub last_state: MinoState
}

//The tetrimino implementation
impl Mino {
    // Creates a new tetrimino
    pub fn new() -> Mino{
        let new_shape: MinoShape = rand::random();
        let new_color: [f32; 4] = get_color(&new_shape);
        let x_pos: f64 = constants::WIN_SIZE_X*0.675;
        let y_pos: f64 = constants::WIN_SIZE_Y*0.18;
        let block_off: [[f64; 2]; 4] = get_block_offset(&new_shape);
        let new_mino: Mino = Mino {
            gl: GlGraphics::new(OpenGL::V4_5),
            shape: new_shape,
            color: new_color,
            blocks: [block::Block::new(new_color, x_pos + block_off[0][0], y_pos + block_off[0][1]),
                     block::Block::new(new_color, x_pos + block_off[1][0], y_pos + block_off[1][1]),
                     block::Block::new(new_color, x_pos + block_off[2][0], y_pos + block_off[2][1]),
                     block::Block::new(new_color, x_pos + block_off[3][0], y_pos + block_off[3][1])],
            pos_x: x_pos,
            pos_y: y_pos,
            block_offset: block_off,
            state: MinoState::Next,
            last_state: MinoState::None};

        println!("MinoShape: {:?}", new_mino.shape);

        return new_mino;
    }

    pub fn render(&mut self, arg: &RenderArgs) { //, args: &RenderArgs) {
        // Render each block in the tetrimino
        for i in 0..self.blocks.len() {
            self.blocks[i].render(arg); //arg);
        }
    }

    // Update tetrimino
    pub fn update(&mut self) {
        // Update the states 
        self.set_state(self.state);
        
        // Update the block position
        self.update_block_pos();
    }

    // Sets the state of a tetrimino
    pub fn set_state(&mut self, new_state: MinoState) {
        self.last_state = self.state;
        self.state = new_state;
    }

    // Set fall start position
    pub fn set_fall_start_pos(&mut self) {
        self.pos_x = constants::FALL_START_POS[0];
        self.pos_y = constants::FALL_START_POS[1];
        println!("Setting fall start position for mino {:?} to (x,y) = ({},{})", self.shape, self.pos_x, self.pos_y);
    }

    // Update block positions
    pub fn update_block_pos(&mut self) {
        for i in 0..self.blocks.len() {
            self.blocks[i].pos_x = self.pos_x + self.block_offset[i][0];
            self.blocks[i].pos_y = self.pos_y + self.block_offset[i][1];
        }
    }
}

// Implement Clone for Mino struct
impl Clone for Mino {
    fn clone (&self) -> Mino {
        //*self
        let clone_mino: Mino = Mino {
            gl: GlGraphics::new(OpenGL::V4_5),
            shape: self.shape,
            color: self.color,
            blocks: [block::Block::new(self.color, self.pos_x + self.block_offset[0][0], self.pos_y + self.block_offset[0][1]),
                     block::Block::new(self.color, self.pos_x + self.block_offset[1][0], self.pos_y + self.block_offset[1][1]),
                     block::Block::new(self.color, self.pos_x + self.block_offset[2][0], self.pos_y + self.block_offset[2][1]),
                     block::Block::new(self.color, self.pos_x + self.block_offset[3][0], self.pos_y + self.block_offset[3][1])],
            pos_x: self.pos_x,
            pos_y: self.pos_y,
            block_offset: self.block_offset,
            state: self.state,
            last_state: self.last_state};
        return clone_mino;
    }
}

// Used to get a color for the tetrimino being generated. Colors tetrimino depending on shape. Source: https://tetris.fandom.com/wiki/Tetromino#I
fn get_color(shape_input: &MinoShape) -> [f32; 4] {
    match shape_input {
        MinoShape::I => constants::LIGHTBLUE,
        MinoShape::O => constants::YELLOW,
        MinoShape::T => constants::PURPLE,
        MinoShape::S => constants::GREEN,
        MinoShape::Z => constants::RED,
        MinoShape::J => constants::BLUE,
        MinoShape::L => constants::ORANGE,
    }
}

// Used to get a color for the tetrimino being generated. Colors tetrimino depending on shape. Source: https://tetris.fandom.com/wiki/Tetromino#I
fn get_block_offset(shape_input: &MinoShape) -> [[f64;2]; 4] {
    let block_dim:f64 = constants::BLOCK_DIM as f64;
    match shape_input {
        MinoShape::I => [[0.0,0.0],
                         [0.0,-block_dim],
                         [0.0,-block_dim*2.0],
                         [0.0,-block_dim*3.0]],
        MinoShape::O => [[0.0,0.0],
                         [0.0,-block_dim],
                         [-block_dim, 0.0],
                         [-block_dim, -block_dim]],
        MinoShape::T => [[0.0,0.0],
                         [-block_dim, 0.0],
                         [-block_dim, -block_dim],
                         [-block_dim*2.0, 0.0]],
        MinoShape::S => [[0.0,0.0],
                        [-block_dim, 0.0],
                        [-block_dim, -block_dim],
                        [-block_dim*2.0, -block_dim]],
        MinoShape::Z => [[0.0,0.0],
                         [0.0,-block_dim],
                         [-block_dim,-block_dim],
                         [-block_dim,-block_dim*2.0]],
        MinoShape::J => [[0.0,0.0],
                         [0.0,-block_dim],
                         [-block_dim, 0.0],
                         [-block_dim*2.0, 0.0]],
        MinoShape::L => [[0.0,0.0],
                         [-block_dim, 0.0],
                         [-block_dim*2.0, 0.0],
                         [-block_dim*2.0, -block_dim]],
    }
}


#[derive(Debug)]
#[derive(Copy, Clone)]
enum MinoShape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

// To be able to get random MinoShapes
impl Distribution<MinoShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MinoShape {
        match rng.gen_range(0, 3) {
            0 => MinoShape::I,
            1 => MinoShape::O,
            2 => MinoShape::T,
            3 => MinoShape::S,
            4 => MinoShape::Z,
            5 => MinoShape::J,
            _ => MinoShape::L,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MinoState {
    Next,
    Falling,
    Hold,
    Still,
    None
}