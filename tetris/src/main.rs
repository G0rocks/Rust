/*
**************************************************************************************************************
Höfundur:       Huldar
Dagsetning:     25. júlí 2020
Forrit:         Tetris í rust
Upplýsingar:    Upphaflegi tetris leikurinn: https://is.wikipedia.org/wiki/Tetris
                Reglurnar um hvað formin heita, liti, stigagjöf og fleira er hægt að finna á 
                ensku síðunni á wikipedia um tetris.
                Nöfnin á tetrimino-unum er að finna hér: https://tetris.fandom.com/wiki/Tetromino

Next job: Make a window
**************************************************************************************************************
*/

// Declare modules
mod functions;  // declare functions.rs as a module
mod tetrimino;  // declare tetrimino.rs as a module. For use with creating tetrimino related things
mod game;       // The game module
mod block;      // The blocks in the tetriminos
mod constants;  // global constants like colours and points and such

// Declare crates
extern crate rand;          // Random crate to generate random numbers to make the tetriminos appear randomly
extern crate rodio;         // Used to play audio - The theme song
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;        // Used to manipulate the game window

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::{RenderEvent, UpdateEvent};      // So we can render
use glutin_window::GlutinWindow;    // Piston gluten window, piston depenedency
use opengl_graphics::{GlGraphics, OpenGL};  // Piston 2D graphics dependency
//use std::fs::File;          // Used to import the audio file for the music
//use std::io::BufReader;     // Used to read data from audio file
//use rodio::Source;          // Used to play audio
//use std::path::Path;        // Used to find the audio file

pub fn main() {
    println!("main function running");

    // Breytur (e. Variables) sem við munum þurfa að nota
    let mut _my_points:u32 = 0;                              // How many points the player has
    let mut _my_level:u32 = 0;                                // What is the current level -> The level determines the fall speed of tetriminos
    let mut _my_lines:u16 = 0;                               // How many lines has the player made disappear
    let mut _enter_to_play_has_been_pressed:bool = false;    // Specifies whether the game has started or not
    let tetris_zone_pos: [usize; 4] = constants::TETRIS_ZONE_POS; // The play zone position
    let next_zone_pos: [usize; 4] = constants::NEXT_ZONE_POS; // The play zone position
    let game_updates_per_sec:u64 = 38;                   // How many times per second the game should update

    println!("Variables declared");

    // start playing music - Tetris ogg file includes the tetris theme song

//    let audio_path = Path::new("audio/Tetris_theme.ogg");
//    let file = File::open(audio_path).unwrap();
//    let source = rodio::Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
//    let device = rodio::default_output_device().unwrap();
//    rodio::play_raw(&device, source.convert_samples());

    
//    println!("Audio started");
    println!("Opening window");


    // Open a window
    // Rendering interface
    let opengl = OpenGL::V3_2;

    // Open a window with a press enter to start (and some other information?)
    //WindoSettings::new{Title T (must be string), Size S (must be array, [width,height])}
    //Coordinates for snake: (x0,y0) = (0,0) --> (x1,y1) --> (X_MAX,Y_MAX)
    let mut window: GlutinWindow = WindowSettings::new("Tetris - Cover by Huldar", (constants::WIN_SIZE_X, constants::WIN_SIZE_Y))
    .fullscreen(false)
    .vsync(true)
    .graphics_api(OpenGL::V3_2)
    .exit_on_esc(true)
    .build()
    .unwrap();

    println!("Window opened");

    // Leyfum notandanum að stækka/minnka gluggann

    // Add a (full program) loop which waits until enter is pressed to start the game and takes care of menus and such

    // Create tetrimino vecotr
    let mino_vector: Vec<tetrimino::Mino> = Vec::new();

    // Creat zone array
    let zone_array: [[i32; constants::GRID_BLOCK_SIZE_X]; constants::GRID_BLOCK_SIZE_Y] = [[0; constants::GRID_BLOCK_SIZE_X]; constants::GRID_BLOCK_SIZE_Y];

    // Create game
    let mut current_game = game::Game {
        gl: GlGraphics::new(opengl),
        minos: mino_vector,
        updates_per_second: game_updates_per_sec,
        fall_counter: 0,
        fall_count_max: 1,
        audio_on: true,
        game_over: false,
        zone_pos: tetris_zone_pos,
        next_pos: next_zone_pos,
        game_zone: zone_array,
    };

    // Firstly we need a new tetrimino, so we add a new one
    current_game.minos.push(tetrimino::Mino::new());


    // Game loop - checks if the game is over
    //while functions::game_is_on(top_line ,tetris_zone_height) {
    //Loop - While Game == notover && make sure the runtime of the loop is controlled by the speed
    let mut events = Events::new(EventSettings::new()).ups(current_game.updates_per_second);
    println!("Game started");
    while let Some(e) = events.next(&mut window) {
        // Render game. Must render before the update event
        if let Some(r) = e.render_args() {
            current_game.render(&r);
        }

        // Setjum tetrimino inn í skjáinn einn í einu
        if let Some(u) = e.update_args() {
            current_game.update();
        }

        // If the game is over, exit the loop
        if current_game.game_over {
            break;
        }
    }

    println!("Game over");
}