extern crate rand;
extern crate rand_pcg;
extern crate sdl2;

mod algebra;
mod bullet;
mod bullet_pool;
mod cannon;
mod collision;
mod enemy;
mod enemy_path;
mod enemy_pool;
mod graphic_object;
mod key_state;
mod player;
mod random_tools;
mod session;
mod time_manager;
mod wave_generator;
mod window_rect;

use session::Session;
use window_rect::WINDOW_SIZE;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::time::SystemTime;

use graphic_object::{LineSegs2f, Polygon2f};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("eyhv", WINDOW_SIZE.x as u32, WINDOW_SIZE.y as u32)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    //canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.present();
    let mut texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(
            None,
            WINDOW_SIZE.x as u32,
            WINDOW_SIZE.y as u32
        ).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut session = Session::new();

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
                    Keycode::LShift => session.proc_key(4, true),
                    Keycode::Z => session.proc_key(5, true),
                    Keycode::Space => session.proc_key(6, true),
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
                    Keycode::LShift => session.proc_key(4, false),
                    Keycode::Z => session.proc_key(5, false),
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

        texture.update(
            None,
            &session.render(),
            WINDOW_SIZE.x as usize * 3,
        );


        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&texture, None, None);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 80));
    }
}
