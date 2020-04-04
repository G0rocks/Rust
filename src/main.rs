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

//Using windows. This command avoids opening the command prompt on execution
//#![windows_subsystem = "windows"]

//External crates
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

//Used dependencies
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;
use std::vec::Vec;
use std::io;
use std::io::prelude::*;

//Declaring constants
const X_MAX: i32 = 31;
const Y_MAX: i32 = 23;
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

//Enum for directions
#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

//Snake struct. Is really just a rectangle that has a direction and moves. The length is not important anymore. Will change it to only refer to the vector
struct Snake {
    gl: GlGraphics,
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
    last_dir: Direction,
}

//Snake implimentation. Has all the functions that the Snake type can use.
impl Snake {
    pub fn render(&mut self, args: &RenderArgs) {
        let square = graphics::rectangle::square((self.pos_x*20) as f64, (self.pos_y*20) as f64, 20_f64);

        self.gl.draw(args.viewport(), |c,gl|{
            let transform = c.transform;

            graphics::rectangle(WHITE, square, transform, gl)
        });
    }

    //Step function (must add something where we make sure that the next snake will follow the one before it)
    fn update(&mut self) {
        match self.dir {
            Direction::Left => self.pos_x = ((self.pos_x-1)%(X_MAX+1)+(X_MAX+1))%(X_MAX+1),      //Weird remainder calculations to change the % operator into modulus so we can leave from one side of the screen and enter on another side
            Direction::Right => self.pos_x = ((self.pos_x+1)%(X_MAX+1)+(X_MAX+1))%(X_MAX+1),
            Direction::Up => self.pos_y = ((self.pos_y-1)%(Y_MAX+1)+(Y_MAX+1))%(Y_MAX+1),
            Direction::Down => self.pos_y = ((self.pos_y+1)%(Y_MAX+1)+(Y_MAX+1))%(Y_MAX+1),
        }
    }
}

//Apple struct
struct Apple {
    gl: GlGraphics,
    pos_x: i32,
    pos_y: i32,
}

impl Apple {
    //render fn
    fn render(&mut self, args: &RenderArgs) {
        let square = graphics::rectangle::square((self.pos_x*20) as f64, (self.pos_y*20) as f64, 20_f64);

        self.gl.draw(args.viewport(), |c,gl|{
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl)
        });
    }

    fn update (&mut self) {
        let mut rng = rand::thread_rng();
        self.pos_x = rng.gen_range(1, X_MAX);
        self.pos_y = rng.gen_range(1, Y_MAX);
    }
}

//This is the game struct
struct Game {
    gl: GlGraphics,
    vector: Vec<Snake>,
    apple: Apple,
    updates_per_second: u64,
    game_over: bool,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK,gl);
        });

        self.apple.render(arg);

        for i in 0..self.vector.len() {
            self.vector[i].render(arg);
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
    
    //Growing snake (possibly need to add something where we talk about the directions). num is the length of the Snake_vector
    fn grow (&mut self, num: u8) {
        let mut new_snake_pos_x = self.vector[self.vector.len()-1].pos_x;
        let mut new_snake_pos_y = self.vector[self.vector.len()-1].pos_y;

        //Depending on the last snakes direction, change the new snakes starting position
        match self.vector[(num as usize)-1].dir.clone() {
            Direction::Left => new_snake_pos_x = ((new_snake_pos_x+1)%(X_MAX+1)+(X_MAX+1))%(X_MAX+1),      //Weird remainder calculations to change the % operator into modulus so we can leave from one side of the screen and enter on another side
            Direction::Right => new_snake_pos_x = ((new_snake_pos_x-1)%(X_MAX+1)+(X_MAX+1))%(X_MAX+1),
            Direction::Up => new_snake_pos_y = ((new_snake_pos_y+1)%(Y_MAX+1)+(Y_MAX+1))%(Y_MAX+1),
            Direction::Down => new_snake_pos_y = ((new_snake_pos_y-1)%(Y_MAX+1)+(Y_MAX+1))%(Y_MAX+1),
        }
        
        let new_snake = Snake { gl: GlGraphics::new(OpenGL::V3_2),
                                pos_x: new_snake_pos_x,
                                pos_y: new_snake_pos_y,
                                dir: self.vector[self.vector.len()-1].last_dir.clone(),
                                last_dir: self.vector[self.vector.len()-1].last_dir.clone()};
        self.vector.push(new_snake)
    }

    fn update(&mut self) {
        let vector_length = self.vector.len();
        //Takes a step forward
        for i in 0..vector_length {
            if i != vector_length-1 && vector_length > 1{
                self.vector[vector_length-1-i].dir = self.vector[vector_length-i-2].last_dir.clone();
            }
            self.vector[vector_length-1-i].update();
        }

        //Set the last direction as the current direction and move on.
        for i in 0..vector_length {
            self.vector[i].last_dir = self.vector[i].dir.clone();
        }

        //Eat apple?
        if self.vector[0].pos_x == self.apple.pos_x && self.vector[0].pos_y == self.apple.pos_y {
            self.grow(self.vector.len() as u8);
            let mut i = 0;
            while i < vector_length {
                //We check if the apple is inside the snake and regenerate it if so.
                if self.apple.pos_x == self.vector[i].pos_x && self.apple.pos_y == self.vector[i].pos_y {
                    self.apple.update();
                    i = 0;
                }
                i += 1;
            }
        }

      //Include if crash into self
        if vector_length > 4 {
            for i in 4..vector_length {
                if self.vector[0].pos_x == self.vector[i].pos_x && self.vector[0].pos_y == self.vector[i].pos_y {
                    self.game_over = true;
                }
            }
        }
    }

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

                //We default to the last direction so if somebody accidentally presses a different button, nothing happens
               _ => self.vector[0].dir.clone()
        }
    }
}

/*
*****************************************************************************************************************************
Main function
*/
fn main() {
    //random
    let mut rng = rand::thread_rng();
    let mut total_updates = 0;

    //No idea what this is or what it does but the code doesn't work withou it :P
    let opengl = OpenGL::V3_2;

    //WindoSettings::new{Title T (must be string), Size S (must be array, [width,height])}
    //Coordinates for snake: (x0,y0) = (0,0) --> (x1,y1) --> (X_MAX,Y_MAX)
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", (640, 480))
    .fullscreen(false)
    .vsync(true)
    .graphics_api(OpenGL::V3_2)
    .exit_on_esc(true)
    .build()
    .unwrap();
    
    //Declaring snake vector. This vector will contain a data type Snake. Which is a cube. For each apple the player gets, we add 1 Snake (type) to this vector
    let mut snake_vector: Vec<Snake> = Vec::new();
    let snake1 = Snake {gl: GlGraphics::new(opengl), pos_x: 10, pos_y: 10, dir: Direction::Right, last_dir: Direction::Right};
    snake_vector.push(snake1);

    //Here the core game resides
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        vector: snake_vector,
        apple: Apple {gl: GlGraphics::new(opengl), pos_x: rng.gen_range(1,X_MAX), pos_y: rng.gen_range(1, Y_MAX)},
        updates_per_second: 15,
        game_over: false,
    };

    //Cool screen with "GAME STARTING" + "3" + "2" + "1" "BEGIN"


    //Loop - While Game == notover && make sure the runtime of the loop is controlled by the speed
    let mut events = Events::new(EventSettings::new()).ups(game.updates_per_second);
    while let Some(e) = events.next(&mut window) {

        //If the event is a render event we will do something. Must render before the update event
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        //If the event is an update we wil do something. u is underscored to signify that we never actually use the variable u
        if let Some(_u) = e.update_args() {
            total_updates += 1;
            game.update();

            //Change speed of snake depending on how long the snake is.            game.updates_per_second = game.snake.length*5;

        }

        //Change direction when button is pressed event call
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    
        //Check if game is over to close the window and break the loop
        if game.game_over {
            break;
        }
    }

    //Print score
    println!("\nYour score was: {}", game.vector.len());

    //Print time
    let time = total_updates/game.updates_per_second;
    println!("\nYour time was: {}", time);
    //When game is over make a cool ASCII "artwork" with a GAME OVER or a YOU WON.
   // if let Some(j) = e.render_args() {
    //    game.game_over_render(&j);
   // }

    //Press any key to quit
    {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
    
        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "Press enter to quit").unwrap();
        stdout.flush().unwrap();
    
        // Read a single byte and discard
        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}