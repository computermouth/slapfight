
use sdl2::rect::Rect;
use sdl2::pixels::Color;

#[derive(Debug)]
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
	pub fn new( 
		x: i32, y: i32, w: u32, h: u32, 
		r: u8, g: u8, b: u8, a: u8)
		-> MyRect {
		
		MyRect {x: x, y: y, w: w, h: h, r: r, g: g, b: b, a: a}
	}
}
