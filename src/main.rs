// Using the external dependencies.
extern crate allegro;
extern crate allegro_acodec;
extern crate allegro_audio;
extern crate allegro_font;
extern crate allegro_primitives;
extern crate allegro_ttf;

use allegro::*;
use allegro_acodec::*;
use allegro_audio::*;
use allegro_font::*;
use allegro_primitives::*;
use allegro_ttf::*;
use player::Player;
use gold::GoldPosition;
use core::*;

mod player;
mod gold;
mod core;

// Creating some constants to add a bit of meaning.
const DISPLAY_WIDTH: i32 = 800;
const DISPLAY_HEIGHT: i32 = 600;
const RECT_THICKNESS: i32 = 5;

/**
 * This function is to check if the Player collided with the gold.
 * If yes then obviously that means the Player picked the gold coin
 * (not exactly a coin but whatever).
 */
fn collided(player: Player) -> bool {
    basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 850,
        DISPLAY_HEIGHT - 550,
        40,
        40,
    ) || basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 850,
        DISPLAY_HEIGHT - 200,
        40,
        40,
    ) || basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 150,
        DISPLAY_HEIGHT - 550,
        40,
        40,
    ) || basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 150,
        DISPLAY_HEIGHT - 200,
        40,
        40,
    )
}

allegro_main! {
    // We need to init the allegro stuffs.
    let core = Core::init().unwrap();
    let primitives = PrimitivesAddon::init(&core).unwrap();
    let font_addon = FontAddon::init(&core).unwrap();
    let audio_addon = AudioAddon::init(&core).unwrap();
    let ttf_addon = TtfAddon::init(&font_addon).unwrap();
    AcodecAddon::init(&audio_addon).unwrap();

    // We need to install/register the keyboard to be able to use it.
    core.install_keyboard().unwrap();
    // Instantiating a new display.
    let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    // Setting the window title (not sure if we need this for this game).
    display.set_window_title("Ecstasy of gold");

    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

    // Loading our custom font from the disk.
    let font = ttf_addon.load_ttf_font("fonts/OpenSans-Regular.ttf",
    36, Flag::zero()).unwrap();

    // Instantiating and registering all the sources that creates events.
    let eq = EventQueue::new(&core).unwrap();
    eq.register_event_source(timer.get_event_source());
    eq.register_event_source(core.get_keyboard_event_source().unwrap());

    let mut redraw = true;
    timer.start();

    // I'm not understanding this part of the sound...
    // let sample = Sample::load(&audio_addon, "sounds/sound_gold.wav").unwrap();
    // let mut sink = Sink::new(&audio_addon).unwrap();
    // let mut stream = AudioStream::load(&audio_addon, "sounds/song_gold.wav").unwrap();
    // stream.set_playmode(Playmode::Loop).unwrap();

    let dark_green = Color::from_rgb_f(0.0, 43.0, 54.0);
    let yellow = Color::from_rgb_f(181.0, 137.0, 0.0);
    let black = Color::from_rgb_f(0.0, 0.0, 0.0);

    let player = Player {
        x: DISPLAY_WIDTH / 2,
        y: DISPLAY_HEIGHT / 2,
        score: 250,
        move_speed: 45,
        steps: 0,
    };

    let done = 0;
    let seconds = 60;
    let gold_position = GoldPosition::rand_gold_position();

    'exit: loop {

        if redraw && eq.is_empty() {
            core.clear_to_color(black);

            let player_score = format!("SCORE: {}", player.score);
            core.draw_text(&font, dark_green,
                10.0, 1.0, FontAlign::Left, &player_score);
            
            let player_steps = format!("STEPS: {}", player.steps);
            core.draw_text(&font, dark_green,
                ((DISPLAY_WIDTH / 2) - 150) as f32, 1.0,
                FontAlign::Left, &player_steps);
            let time_remaining = format!("TIME REMAINING: {}", seconds);
            core.draw_text(&font, dark_green,
                (DISPLAY_WIDTH - 360) as f32, 1.0,
                FontAlign::Left, &time_remaining);
            primitives.draw_line(1.0, 50.0,
                (DISPLAY_WIDTH - 1) as f32, 50.0, dark_green,
                RECT_THICKNESS as f32);

            primitives.draw_rectangle(80.0, 140.0, 280.0, 340.0,
                dark_green, RECT_THICKNESS as f32);
            primitives.draw_rectangle(80.0, 480.0, 280.0, 680.0,
                dark_green, RECT_THICKNESS as f32);

            primitives.draw_rectangle((DISPLAY_WIDTH - 80) as f32,
            140.0, (DISPLAY_WIDTH - 280) as f32,
            340.0, dark_green, RECT_THICKNESS as f32);

            primitives.draw_rectangle((DISPLAY_WIDTH - 80) as f32,
            480.0, (DISPLAY_WIDTH - 280) as f32,
            680.0, dark_green, RECT_THICKNESS as f32);

            core.draw_text(&font, dark_green, player.x as f32, player.y as f32, FontAlign::Centre, "X");

            core.flip_display();
            redraw = false;
        }

        match eq.wait_for_event() {
            DisplayClose{..} => break 'exit,
            KeyDown{keycode: k, ..} if k == KeyCode::Escape => {
                println!("bye");
                break 'exit;
            },
            TimerTick{..} => redraw = true,
            _ => (),
        }
    }
}
