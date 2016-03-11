use sdl2;
use sdl2::render;
use sdl2::event::Event;

use logo_shapes;
use game_state::GameState;

use modrect;
use shapes;

const END : u32 = 300;

//pub fn logo_scene( logotuple: ( render::Renderer, sdl2::sdl::EventPump, GameState, u32) ) -> ( render::Renderer, sdl2::sdl::EventPump, GameState, u32) {
pub fn logo_scene( mut grenderer: render::Renderer ) -> render::Renderer {
	
	//~ let mut grenderer : render::Renderer = logotuple.0;
	//~ let mut event_pump : sdl2::sdl::EventPump = logotuple.1; 
	//~ let mut state : GameState = logotuple.2;
	//~ let mut logo_timer : u32 = logotuple.3;
	
	grenderer = modrect::draw(grenderer, shapes::RED);
	grenderer = modrect::draw(grenderer, shapes::GREEN);
	grenderer = modrect::draw(grenderer, shapes::PURP);
	grenderer = modrect::draw(grenderer, logo_shapes::NTC_002);
	
	//~ let returntuple = (grenderer, event_pump, state, logo_timer);
	//~ returntuple
	grenderer
}
