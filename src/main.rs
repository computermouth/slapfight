extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

struct MyRect {
	x: i32,
	y: i32,
	w: u32,
	h: u32,
}

impl MyRect {
	fn rectin(xin: i32, yin: i32, win: u32, hin: u32) -> MyRect {
		MyRect {x: xin, y: yin, w: win, h: hin,}
	}
	fn rectout(&self) -> Rect {
		Rect::new(self.x, self.y, self.w, self.h)
	}
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        //.opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    
	'running: loop {
		
		renderer.set_draw_color(Color::RGBA(0, 0, 0, 255));
		renderer.clear();
		renderer.set_draw_color(Color::RGBA(125, 0, 125, 255));
		
		//let rect = Rect::new(320, 240, 128, 128);
		//renderer.fill_rect(rect).unwrap();
		
		let purp = MyRect::rectin(320, 240, 128, 128);
		renderer.fill_rect(purp.rectout()).unwrap();
		
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
