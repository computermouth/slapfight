
use sdl2::render;

use rectangle;

pub fn draw(mr : rectangle::MyRect, mut grenderer: render::Renderer) 
	-> render::Renderer {
	grenderer.set_draw_color(mr.colorout());
	grenderer.fill_rect(mr.rectout()).unwrap();
		
	grenderer
}
