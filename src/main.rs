extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod modrect;
mod shapes;
//mod game_state;
//mod action;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SLAPFIGHT", 600, 440)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
	'running: loop {
		
		renderer.set_draw_color(Color::RGBA(0, 0, 0, 255));
		renderer.clear();
		
		let purp = modrect::MyRect::new()
			.x(120)
			.y(120)
			.w(120)
			.h(120)
			.r(200)
			.g(0)
			.b(200)
			.a(120)
			.finalize();
			

		renderer = modrect::draw(renderer, purp);
		
		let green = modrect::MyRect::new()
			.x(200)
			.y(200)
			.w(120)
			.h(120)
			.r(0)
			.g(200)
			.b(0)
			.a(120)
			.finalize();
			
		//~ renderer.set_draw_color(green.colorout());
		//~ renderer.fill_rect(green.rectout()).unwrap();

		renderer = modrect::draw(renderer, green);
		renderer = modrect::draw(renderer, shapes::RED);
		
		//~ let green = modrect::MyRect::new(100, 100, 100, 100, 0, 150, 0, 255);
		//~ renderer.set_draw_color(green.colorout());
		//~ renderer.fill_rect(green.rectout()).unwrap();
		//~ green.draw();
		
		renderer.present();

		let mut event_pump = sdl_context.event_pump().unwrap();

    
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}
