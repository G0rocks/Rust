/*
This is Huldars attempt at creating a game of snake using the programming language Rust
Huldar is a total noob but will try his best ^_^
*******************************************************************************************
Creation date: 26.3.2020 at 19:40 (UTC+1)
Author: Huldar
License: Anybody can do pretty much whatever they want with this code.
*******************************************************************************************

How it will work.
You run the program and the _snake_ will start moving.
The snake can go through a wall on the left side and emerge from the right side
Every now and then an "apple" will appear on the screen which if the snake "eats" it, then the snake will grow by 1 "step"
The user can use the arrow keys to move the snake around.
As the snake "eats" more "apples" he not only grows but also gains speed.
The game is over when the snake touches itself or if there is no more space on the screen for the snake.
*/

//External crates
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

//Used dependencies
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

//Eat apple function

//Step function

fn main() {
    //Variable declaration
        //Speed
        //Length
        //Direction?
        //Boolean game over?


    //Stuff from random youtube video: https://www.youtube.com/watch?v=HCwMb0KslX8
    let opengl = OpenGL::V3_2;

    //WindoSettings::new{Title T (must be string), Size S (must be array, [width,height])}
    let mut window = WindowSettings::new(
        "Snake Game",
        [512,512]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    //Cool screen with "GAME STARTING" + "3" + "2" + "1" "BEGIN"

    //Loop - While Game == notover && make sure the runtime of the loop is controlled by the speed

        //Change direction?

        //Step forward
    
            //Include if "crash" into wall
    
            //Include if apple was eaten
    
            //Include if crash into self

        //Check if game is over to break loop

    //When game is over make a cool ASCII "artwork" with a GAME OVER or a YOU WON
    
    println!("If this prints out, that means the program is working :)");
}
