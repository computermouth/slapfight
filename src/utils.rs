
use sdl2::render;

//~ use anim::shape::rect::rectangle;
use rectangle;

pub fn draw(mr : rectangle::MyRect, mut grenderer: render::Renderer) 
	-> render::Renderer {
	grenderer.set_draw_color(mr.colorout());
	grenderer.fill_rect(mr.rectout()).unwrap();
		
	grenderer
}
