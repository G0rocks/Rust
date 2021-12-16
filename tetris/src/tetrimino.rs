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

use opengl_graphics::GlGraphics;

// Defines the tetrimino structure
pub struct Mino {
    gl: GlGraphics,
    pos_x: i32,
    pos_y: i32,
}

//The tetrimino implementation
impl Mino {
    pub fn render(&mut self) { //, args: &RenderArgs) {
        println!("Rendering tetrimino");
    }

}