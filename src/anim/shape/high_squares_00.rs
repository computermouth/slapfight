
use sdl2::render;

use anim::shape::rect::rectangle;
use utils;


pub struct Frame {
	red: rectangle::MyRect,
	green: rectangle::MyRect,
	purp: rectangle::MyRect,
}


impl Frame {
	pub fn new() -> Frame {
		Frame {
			red : rectangle::MyRect::new(
				200, 0, 120, 120, 
				240, 173, 109, 255
			),
			green : rectangle::MyRect::new(
				300, 100, 120, 120,
				64, 64, 64, 255 
			),
			purp : rectangle::MyRect::new(
				400, 200, 120, 120, 
				175, 194, 255, 255
			),
		}
	}
}

pub fn draw(ts: Frame, mut renderer: render::Renderer) -> render::Renderer {
	renderer = utils::draw(ts.red, renderer);
	renderer = utils::draw(ts.green, renderer);
	renderer = utils::draw(ts.purp, renderer);
	renderer
}
