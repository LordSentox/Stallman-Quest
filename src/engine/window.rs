/*
 * This file is part of Stallman-Quest, a game about fighting against incredible odds.
 * Copyright (C) 2015 Arne Dussin
 *
 * Stallman-Quest is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Stallman-Quest is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Stallman-Quest.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate sdl2;

pub struct Window
{
	pub sdl_window: sdl2::video::Window,
	pub sdl_context: sdl2::Sdl
}

fn create_context () -> sdl2::Sdl
{
	sdl2::init().everything().unwrap()
}

impl Window {

	pub fn new (title: &str, width: u32, height: u32) -> Result <Window, String>
	{
		let sdl_context = create_context();

		let sdl_window = match sdl_context.window(title, width, height).opengl().build() {
			Ok (sdl_window) => sdl_window,
			Err (err) => return Err (err)
		};

		Ok (Window {
			sdl_window: sdl_window,
			sdl_context: sdl_context
		})
	}

	pub fn new_fullscreen (title: &str) -> Result <Window, String>
	{
		let sdl_context = create_context();

		let sdl_window = match sdl_context.window(title, 0, 0).opengl().fullscreen_desktop().build() {
			Ok (sdl_window) => sdl_window,
			Err (err) => return Err (err)
		};

		Ok (Window {
			sdl_window: sdl_window,
			sdl_context: sdl_context
		})
	}
}
