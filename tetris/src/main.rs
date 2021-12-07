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

mod functions;  // declare functions.rs as a module
mod tetrimino;  // declare tetrimino.rs as a module. For use with creating tetrimino related things

// Declare crates
extern crate rand;          // Random crate to generate random numbers to make the tetriminos appear randomly
extern crate rodio;         // Used to play audio - The theme song
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;        // Used to manipulate the game window

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;           // Used to get input from user
use glutin_window::GlutinWindow;    // Piston gluten window, piston depenedency
use opengl_graphics::{GlGraphics, OpenGL};  // Piston 2D graphics dependency
use std::fs::File;          // Used to import the audio file for the music
use std::io::BufReader;     // Used to read data from audio file
use rodio::Source;          // Used to play audio
use std::path::Path;        // Used to find the audio file

fn main() {
    #![windows_subsystem = "windows"]   // To avoid having a console pop up on windows release

    // Breytur (e. Variables) sem við munum þurfa að nota
    let mut my_points:u32 = 0;                              // How many points the player has
    let mut my_level:u8 = 0;                                // What is the current level -> The level determines the fall speed of tetriminos
    let mut my_lines:u16 = 0;                               // How many lines has the player made disappear
    let block_width:u8 = 20;                                // Number of pixels in the width of a square (squares are used to create tetriminos)
    let block_height:u8 = 20;                               // Number of pixels in the height of a square (squares are used to create tetriminos)
    let mut enter_to_play_has_been_pressed:bool = false;    // Specifies whether the game has started or not
    let tetris_zone_height:u8 = 40;                         // Specifies the height of the tetris zone in blocks - Standard is a 10x40 playing field (ref: https://tetris.fandom.com/wiki/Playfield#:~:text=The%20Tetris%20Guideline%20specifies%20a,the%20bottom%20of%20row%2021.)
    let tetris_zone_width:u8 = 10;                          // Specifies the width of the tetris zone in blocks
    let mut top_line:u8 = 0;                                // Tells us how high the currently placed blocks reach
    
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
    let points_1_line:u8 = 100;
    let points_2_lines:u8 = 250;
    let points_3_lines:u16 = 400;
    let points_tetris:u16 = 800;
    let points_double_tetris:u16 = 1200;
    let points_per_square_hard_drop:u8 = 10;
    
    // Colours
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const YELLOW: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const GREEN: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLUE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const PURPLE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const ORANGE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const LIGHTBLUE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    


    // start playing music - Tetris ogg file includes the tetris theme song

    let path = Path::new("Tetris_theme.ogg");
    let file = File::open("Tetris_theme.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
    let device = rodio::default_output_device().unwrap();
    rodio::play_raw(&device, source.convert_samples());

    // If m or M button is pressed, mute/unmute music
    // event button 'm' is pressed
    // If mute, unmute, if unmuted, mute.
    let mute_event = orbtk::shell::prelude::KeyEvent {
      key: orbtk::shell::event::Key::M(true),
      state: orbtk::shell::event::ButtonState::Down,
      text: String::from("functions::mute_audio")
    };


    // Open a window
    // Rendering interface
    let opengl = OpenGL::V3_2;

    // Open a window with a press enter to start (and some other information?)
    //WindoSettings::new{Title T (must be string), Size S (must be array, [width,height])}
    //Coordinates for snake: (x0,y0) = (0,0) --> (x1,y1) --> (X_MAX,Y_MAX)
    let mut window: GlutinWindow = WindowSettings::new("Tetris - Cover by Huldar", (450, 580))
    .fullscreen(false)
    .vsync(true)
    .graphics_api(OpenGL::V3_2)
    .exit_on_esc(true)
    .build()
    .unwrap();

    //Here the core game resides
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        updates_per_second: 10,
        game_over: false,
    };


    // Run application


    // Leyfum notandanum að stækka/minnka gluggann

    // Add a loop which waits until enter is pressed to start the game

    // Game loop - checks if the game is over
    while functions::game_is_on(top_line ,tetris_zone_height) {
        // Teiknum leiðbeiningar fyrir neðan tetris zone-ið

        // Sínum á hvaða borði (e. level) leikmaðurinn er

        // Sínum fjölda lína sem leikmaðurinn hefur tekist að láta hverfa

        // Setjum tetrimino inn í skjáinn einn í einu

        // Látum tetrimino-ana detta, einn í einu, rólega niður

        // Snúum tetrimino-inum þegar leikmaðurinn ýtir á upp örina

        top_line = tetris_zone_height+3;
    }

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