extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;

mod utils;
mod anim;
use anim::{high_squares_a00};

//~ mod rectangle;
//~ mod three_squares_00;
//~ mod three_squares_01;
//~ mod three_squares_02;
//~ mod three_squares_a00;
//~ mod high_squares_00;
//~ mod high_squares_01;
//~ mod high_squares_02;
//~ mod high_squares_a00;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("SLAPFIGHT", 640, 440)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
		.present_vsync()
		.build()
		.unwrap();
	
	let mut state: u8 = 0;
	
	'running: loop {
		
		renderer.set_draw_color(Color::RGBA(30, 30, 39, 255));
		renderer.clear();
		let mut event_pump = sdl_context.event_pump().unwrap();
		
		//~ let ts : three_squares_a00::Anim = three_squares_a00::Anim::new();
		let hs : high_squares_a00::Anim = high_squares_a00::Anim::new();
		//~ renderer = ts.red(renderer);
		//~ renderer = three_squares_a00::draw(state, ts, renderer);
		renderer = high_squares_a00::draw(state, hs, renderer);
		
		renderer.present();
	
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    break 'running
                },
                _ => {}
            }
        }
        
        if state == 89 {
			state = 0;
		}else{
			state += 1;
		}
    }
    
    
}
