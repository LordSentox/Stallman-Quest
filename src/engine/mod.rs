extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::render::Renderer;

pub trait Drawable {
	fn draw (&self, renderer: &mut Renderer);
}

pub struct Game {
	window: Window,
	sdl_context: Sdl,

	input_handlers: Vec <Box <InputHandler>>,
	locked: bool
}

pub struct Window {
	pub sdl_window: sdl2::video::Window
}

/// A class that implements this trait and is registered in the Sdl-Context, i.e. the instance of
/// Game that is native to the Program by calling the method register() before(!) beginning the
/// listening-process will be notified whenever an event occurs.
pub trait InputHandler {
	fn register (mut self, mut game: Game);
	fn handle (&mut self, event: &Event);
}

pub mod drawable;
pub mod game;
pub mod window;
