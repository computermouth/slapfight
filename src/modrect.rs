
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render;

pub struct MyRect {
	pub x: i32,
	pub y: i32,
	pub w: u32,
	pub h: u32,
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl MyRect {	
	pub fn rectout(&self) -> Rect {
		Rect::new(self.x, self.y, self.w, self.h)
	}
	pub fn colorout(&self) -> Color {
		Color::RGBA(self.r, self.g, self.b, self.a)
	}
}


pub fn draw(mut grenderer: render::Renderer, mr: MyRect) -> render::Renderer {
	grenderer.set_draw_color(mr.colorout());
	grenderer.fill_rect(mr.rectout()).unwrap();
	
	grenderer
}

pub const PINKR : u8 = 255;
pub const PINKG : u8 = 139;
pub const PINKB : u8 = 255; 
