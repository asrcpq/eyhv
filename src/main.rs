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
use window_rect::WINDOW_RECT;

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
    let window_size = WINDOW_RECT.get_size();

    let window = video_subsystem
        .window("eyhv", window_size.x as u32, window_size.y as u32)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // draw after tick
        for graphic_object in session.graphic_object_iter() {
            if let Some(line_segs) = graphic_object.as_any().downcast_ref::<LineSegs2f>() {
                let color = Color::RGBA(
                    (line_segs.color[0] * 255.) as u8,
                    (line_segs.color[1] * 255.) as u8,
                    (line_segs.color[2] * 255.) as u8,
                    (line_segs.color[3] * 255.) as u8,
                );
                let mut iter = line_segs.vertices.iter();
                let mut last_vertex = iter.next().unwrap();
                while match iter.next() {
                    None => false,
                    Some(vertex) => {
                        canvas
                            .aa_line(
                                last_vertex.x as i16,
                                last_vertex.y as i16,
                                vertex.x as i16,
                                vertex.y as i16,
                                color,
                            )
                            .unwrap();
                        last_vertex = vertex;
                        true
                    }
                } {}
            } else if let Some(polygon) = graphic_object.as_any().downcast_ref::<Polygon2f>() {
                let color = Color::RGBA(
                    (polygon.color[0] * 255.) as u8,
                    (polygon.color[1] * 255.) as u8,
                    (polygon.color[2] * 255.) as u8,
                    (polygon.color[3] * 255.) as u8,
                );
                canvas.filled_polygon(
                    polygon
                        .vertices
                        .iter()
                        .map(|x| x.x as i16)
                        .collect::<Vec<i16>>()
                        .as_slice(),
                    polygon
                        .vertices
                        .iter()
                        .map(|x| x.y as i16)
                        .collect::<Vec<i16>>()
                        .as_slice(),
                    color,
                ).unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 80));
    }
}
