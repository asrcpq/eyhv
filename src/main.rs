extern crate sdl2;

mod algebra;
mod graphic_object;
mod key_state;
mod player;
mod session;
mod time_manager;
mod bullet;
mod cannon;
mod bullet_pool;

use algebra::{Point2f, Rect2f};
use session::Session;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
use std::time::SystemTime;

use graphic_object::GraphicObject;

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

    let window_rd = Point2f::from_floats(500., 700.);

    let window = video_subsystem
        .window("eyhv", window_rd.x as u32, window_rd.y as u32)
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

    let mut session = Session::new(Rect2f::from_point2fs(Point2f::new(), window_rd));

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

        for graphic_object in session.graphic_object_iter() {
            match graphic_object {
                GraphicObject::Polygon(_) => unimplemented!(),
                GraphicObject::LineSegs(line_segs) => {
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
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 80));
    }
}
