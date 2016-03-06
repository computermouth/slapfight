extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod modrect;
mod shapes;
mod game_state;
mod logo;
mod logo_shapes;

use game_state::GameState;

pub fn main() {
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
    
    let mut state = GameState::Logo;
    
	'running: loop {
		
		renderer.set_draw_color(Color::RGBA(30, 30, 39, 255));
		renderer.clear();
		
		let mut event_pump = sdl_context.event_pump().unwrap();
		
		let mut some: u8 = 12;
		
		match state {
			GameState::Logo		=> renderer = logo::logo_scene(renderer),
			GameState::Intro	=> println!("intro"),
			GameState::Play		=> println!("play"),
			GameState::Over		=> println!("over"),
		}
		
		renderer.present();

		

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
