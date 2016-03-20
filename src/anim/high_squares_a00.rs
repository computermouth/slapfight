
use sdl2::render;

use anim::shape::high_squares_00;
use anim::shape::high_squares_01;
use anim::shape::high_squares_02;


pub struct Anim {
	f_00: high_squares_00::Frame,
	f_01: high_squares_01::Frame,
	f_02: high_squares_02::Frame,
}

impl Anim {
	pub fn new() -> Anim {
		Anim {
			f_00 : high_squares_00::Frame::new(),
			f_01 : high_squares_01::Frame::new(),
			f_02 : high_squares_02::Frame::new(),
		}
	}
}

pub fn draw(state: u8, ts: Anim, mut renderer: render::Renderer) -> render::Renderer {

	renderer = match state {
		0 ... 29 => high_squares_00::draw(ts.f_00, renderer),
		30 ... 59 => high_squares_01::draw(ts.f_01, renderer),
		60 ... 89 => high_squares_02::draw(ts.f_02, renderer),
		_ => renderer
	};
	
	renderer
}
