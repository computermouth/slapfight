
use sdl2::render;

use rectangle;
use utils;


pub struct Frame {
	red: rectangle::MyRect,
	green: rectangle::MyRect,
	purp: rectangle::MyRect,
}

impl Frame {
	pub fn new() -> Frame {
		Frame {
			green : rectangle::MyRect::new(
				100, 100, 120, 120,
				207, 191, 173, 255  
			),
			purp : rectangle::MyRect::new(
				200, 200, 120, 120,
				139, 139, 205, 255
			),
			red : rectangle::MyRect::new(
				300, 300, 120, 120, 
				255, 139, 255, 255
			),
		}
	}
}

pub fn draw(ts: Frame, mut renderer: render::Renderer) -> render::Renderer {
	renderer = utils::draw(ts.green, renderer);
	renderer = utils::draw(ts.purp, renderer);
	renderer = utils::draw(ts.red, renderer);
	renderer
}
