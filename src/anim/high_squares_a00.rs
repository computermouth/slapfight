
use sdl2::render;

use anim::shape::high_squares::{high_squares_00, high_squares_01, high_squares_02};

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

pub fn draw(state: u16, ts: Anim, mut renderer: render::Renderer) -> render::Renderer {

	renderer = match state {
		90 ... 119 => high_squares_00::draw(ts.f_00, renderer),
		120 ... 149 => high_squares_01::draw(ts.f_01, renderer),
		150 ... 179 => high_squares_02::draw(ts.f_02, renderer),
		180 ... 209 => high_squares_00::draw(ts.f_00, renderer),
		210 ... 239 => high_squares_01::draw(ts.f_01, renderer),
		240 ... 269 => high_squares_02::draw(ts.f_02, renderer),
		_ => renderer
	};
	
	renderer
}
