extern crate sdl2;

use sdl2::Sdl;

struct Drawable;

struct Game {
	window: Window,
	sdl_context: Sdl,

	input_handlers: Vec <Box <InputHandler>>,
	locked: bool
}

pub struct Window {
	pub sdl_window: sdl2::video::Window
}

pub mod drawable;
pub mod game;
pub mod input_handler;
pub mod window;
