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

mod functions; //declare functions.rs as a module
mod tetrimino;  //declare tetrimino.rs as a module. For use with creating tetrimino related things

//Declare crates
extern crate rand;          //Random crate to generate random numbers to make the tetriminos appear randomly
extern crate rodio;                //Used to play audio - The theme song
use orbtk::prelude::*;                  //Used for creating and managing the game window
use std::fs::File;          //Used to import the audio file for the music
use std::io::BufReader;     //Used to read data from audio file
use rodio::Source;          //Used to play audio
use std::path::Path;        //Used to find the audio file

fn main() {
    //Breytur (e. Variables) sem við munum þurfa að nota
    let mut my_points:u32 = 0;              //How many points the player has
    let mut my_level:u8 = 0;                //What is the current level -> The level determines the fall speed of tetriminos
    let mut my_lines:u16 = 0;               //How many lines has the player made disappear
    let block_width:u8 = 20;           //Number of pixels in the width of a square (squares are used to create tetriminos)
    let block_height:u8 = 20;          //Number of pixels in the height of a square (squares are used to create tetriminos)
    let mut enter_to_play_has_been_pressed:bool = false;     //Specifies whether the game has started or not
    let tetris_zone_height:u8 = 40;                 //Specifies the height of the tetris zone in blocks - Standard is a 10x40 playing field (ref: https://tetris.fandom.com/wiki/Playfield#:~:text=The%20Tetris%20Guideline%20specifies%20a,the%20bottom%20of%20row%2021.)
    let tetris_zone_width:u8 = 10;                  //Specifies the width of the tetris zone in blocks
    let mut top_line:u8 = 0;                        //Tells us how high the currently placed blocks reach
    
    // Stigagjöf (e. point system)
    /*
    Fyrir 1 línu
    100 stig
    Fyrir 2 línur
    250 stig
    Fyrir 3 línur
    400 stig
    Fyrir 4 línur (einnig þekkt sem Tetris)
    800 stig
    Fyrir að fá Tetris tvisvar í röð
    1200 stig (fást fyrir seinna Tetris-ið)
    Þegar leikmaður leggur tetrimino niður með því að hard drop-pa
    10 stig fyrir hvern reit sem tetrimino-inn féll niður um í hard dropp-inu
    */
    let points_1_line:u8 = 100;
    let points_2_lines:u8 = 250;
    let points_3_lines:u16 = 400;
    let points_tetris:u16 = 800;
    let points_double_tetris:u16 = 1200;
    let points_per_square_hard_drop:u8 = 10;
    
    //start playing music - Tetris ogg file includes the tetris theme song
    //If m or M button is pressed, mute/unmute music
    let path = Path::new("Tetris_theme.ogg");
    let file = File::open("Tetris_theme.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let device = rodio::default_output_device().unwrap();
    rodio::play_raw(&device, source.convert_samples());

    // Open a window
    //Open a window with a press enter to start (and some other information?)
    Application::new()
    .window(|ctx| {
        Window::new()
            .title("Tetris - Cover by Huldar")
            .position((100.0, 100.0))
            .size(450.0, 580.0)
            .build(ctx)
    })
    .run();

    //Game loop - checks if the game is over


    /*
    while functions::game_is_on(top_line ,tetris_zone_height) {
    }
    */
}


//From tetris.py
/*
# Declare dependencies
gluggi_icon = pyglet.image.load("assets\\images\\icon.png")

# Búum til skjá og setjum leikskjá (e. The Tetris Zone) inn í hann
gluggi = pyglet.window.Window()
gluggi.set_icon(gluggi_icon)               # Setjum lítið icon sem táknar forritið

@gluggi.event
def on_draw():
    glClear(GL_COLOR_BUFFER_BIT)
    glLoadIdentity()

    # Gerum upphafsskjá sem leyfir þér að ýta á "Enter" til að "Spila"
    if (enter_to_play) :
        # Teiknum tetris zone-ið
        glBegin(GL_LINE_LOOP)
        glVertex2f(0.1*gluggi.width, 0.15*gluggi.height)
        glVertex2f(0.1*gluggi.width, 0.85*gluggi.height)
        glVertex2f(0.5*gluggi.width, 0.85*gluggi.height)
        glVertex2f(0.5*gluggi.width, 0.15*gluggi.height)
        glEnd()

            # Teiknum tetrimino-inn í tetris zone-inu
        #my_mino.draw()

            # Teiknum leiðbeiningar fyrir neðan tetris zone-ið
        instructions_label = pyglet.text.Label(("Up = Rotate    Down = Soft drop    L/R = Move sideways     Space = Hard drop"),
            font_name='Times New Roman',
            font_size=12,
            x=0.1*gluggi.width, y=0.1*gluggi.height,
            anchor_x='left', anchor_y='center')
        instructions_label.draw()

        # Teiknum hliðarglugga
        glBegin(GL_LINE_LOOP)
        glVertex2f(0.65*gluggi.width, 0.55*gluggi.height)
        glVertex2f(0.65*gluggi.width, 0.85*gluggi.height)
        glVertex2f(0.85*gluggi.width, 0.85*gluggi.height)
        glVertex2f(0.85*gluggi.width, 0.55*gluggi.height)
        glEnd()

            # Teiknum innan í hliðargluggann tetrimino-inn sem er næstur. Tetrimino-ar eru valdir af handahófi
            # next_tetrimino = randint(1, 7)
            # Sínum fyrir neðan gluggann stigafjölda
        point_label = pyglet.text.Label(("Points : "+ str(my_points)),
            font_name='Times New Roman',
            font_size=16,
            x=0.75*gluggi.width, y=0.45*gluggi.height,
            anchor_x='center', anchor_y='center')
        point_label.draw()
            # Sínum á hvaða borði (e. level) leikmaðurinn er
        level_label = pyglet.text.Label(("Level : "+ str(my_level)),
            font_name='Times New Roman',
            font_size=16,
            x=0.75*gluggi.width, y=0.4*gluggi.height,
            anchor_x='center', anchor_y='center')
        level_label.draw()

            # Sínum fjölda lína sem leikmaðurinn hefur tekist að láta hverfa
        line_counter_label = pyglet.text.Label(("Lines : "+ str(my_lines)),
            font_name='Times New Roman',
            font_size=16,
            x=0.75*gluggi.width, y=0.35*gluggi.height,
            anchor_x='center', anchor_y='center')
        line_counter_label.draw()
    else:
        tetris_logo_label = pyglet.text.Label(("T E T R I S"),
            font_name='Algerian',
            font_size=42,
            x=0.5*gluggi.width, y=0.6*gluggi.height,
            anchor_x='center', anchor_y='center')
        tetris_logo_label.draw()

        enter_to_play_label = pyglet.text.Label(("Pres enter to play"),
            font_name='Times New Roman',
            font_size=32,
            x=0.5*gluggi.width, y=0.3*gluggi.height,
            anchor_x='center', anchor_y='center')
        enter_to_play_label.draw()

        author_note_label = pyglet.text.Label(("Made by Huldar"),
            font_name='Times New Roman',
            font_size=16,
            x=0.9*gluggi.width, y=0.05*gluggi.height,
            anchor_x='right', anchor_y='center')
        author_note_label.draw()


# Leyfum notandanum að stækka/minnka gluggann

# Setjum tetrimino inn í skjáinn einn í einu

# Látum tetrimino-ana detta, einn í einu, rólega niður

# Snúum tetrimino-inum þegar leikmaðurinn ýtir á upp örina

# Færum tetrimino-inn til hægri/vinstri þegar leikmaðurinn ýtir á hægri/vinstri örvatakkana
@gluggi.event()
def on_key_press(key, modifiers):
    global enter_to_play
    if (key == pyglet.window.key.ENTER) :
        enter_to_play = True

# Látum tetrimino-inn detta hratt niður á meðan leikmaðurinn heldur niðri niður örvatakkanum

# Setja þá alla leið niður ef leikmaður ýtir á bilslánna - Þekkt sem "Hard drop"

# Láta tetrimino-a stoppa þegar þeir rekast á annan tetrimino fyrir neðan sig
    # Setja næsta tetrimino af stað þegar núverandi tetrimino stoppar
    # Láta leikinn enda þegar tetrimino er stoppar og er fyrir ofan toppinn á skjánum
        # Þegar leikurinn endar --> Sýna "GAME OVER" og stig leikmanns

# Þegar tetriminoarnir mynda heila, óbrotna, lárétta línu yfir allan leikskjáinn þá hverfa þeira og leikmaðurinn fær
# stig í samræmi við hversu margar línur hurfu samtímis.
    # Láta alla kubba fyrir ofan línuna detta niður þegar línan/línurnar hverfa
    # Færum leikmann á næsta borð (e. level) þegar leikmaður nær ákveðið mörgum stigum. Hraðinn eykst eftir því sem þú kemst á hærra borð

# Keyrum forritið í lúppu (e. loop)
pyglet.app.run()
*/