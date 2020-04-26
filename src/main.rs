extern crate sdl2;

mod algebra;
mod graphic_object;
mod key_state;
mod objects;
mod session;
use session::Session;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use std::time::SystemTime;

use graphic_object::GraphicObject;

pub fn main() {
    let mut arg_collect: Vec<String> = std::env::args().collect();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("eyhv", 500, 750)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut session = Session::new(arg_collect.pop().unwrap());

    let mut last_time = SystemTime::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Left => session.proc_key(0, true),
                    Keycode::Up => session.proc_key(1, true),
                    Keycode::Right => session.proc_key(2, true),
                    Keycode::Down => session.proc_key(3, true),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Left => session.proc_key(0, false),
                    Keycode::Up => session.proc_key(1, false),
                    Keycode::Right => session.proc_key(2, false),
                    Keycode::Down => session.proc_key(3, false),
                    _ => (),
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        let current_time = SystemTime::now();
        let duration_secs = current_time
            .duration_since(last_time)
            .expect("Time error")
            .as_secs_f32();
        // println!("{}", 1. / duration_secs); // print fps
        last_time = current_time;
        session.tick(duration_secs);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.draw_rect(Rect::new(50, 50, 100, 60)).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 80));
    }
}
