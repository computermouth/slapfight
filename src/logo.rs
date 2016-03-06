
use sdl2::render;

use logo_shapes;

use modrect;
use shapes;

static mut FRAME : u32 = 0;
const END : u32 = 300;

pub fn logo_scene(mut grenderer: render::Renderer) -> render::Renderer {
	
	unsafe{ 
		if FRAME < END {
			println!("logo_scene");
			
			grenderer = modrect::draw(grenderer, shapes::RED);
			grenderer = modrect::draw(grenderer, shapes::GREEN);
			grenderer = modrect::draw(grenderer, shapes::PURP);
			
			//~ 
			//~ for x in 1..logo_shapes::NTC_SLICE.len(){
				//~ if logo_shapes::NTC_SLICE[x].a < 255 {
					//~ logo_shapes::NTC_SLICE[x].a += 4;
				//~ }
				//~ grenderer = modrect::draw(grenderer, logo_shapes::NTC_SLICE[x])
			//~ }
			//~ 
			//~ 
			
			FRAME+=1;
		}
	}
	
	grenderer
}
