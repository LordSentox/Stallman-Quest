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

use sdl2::{Sdl};
use sdl2::event::{EventPump, Event};

use engine::window::{Window};
use engine::input_handler::{InputHandler};

pub struct Game
{
    window: Window,
    sdl_context: Sdl,

    // XXX: Is it possible to somehow create a Vector, but with a required Sized type you still can
    // overload or similar?
    input_handlers: Vec <InputHandler>,
    locked: bool
}

impl Game {

    pub fn new (name: &str, width: u32, height: u32) -> Result <Game, String>
    {
        let this = Game {
            sdl_context: sdl2::init().everything().unwrap(),

            input_handlers: vec![],
            locked: false
        };

        this.window = match Window::new (this, name, width, height) {
            Ok (window) => window,
            Err (err) => return Err (err)
        };

        Ok (this)
    }

    pub fn window (&self) -> &Window {
        &self.window
    }

    ///
    // TODO: It might be nice to make it possible to still register an event-handler while the loop
    // is running, which requires more sophisticated mechanisms concerning thread-safety.
    pub fn register_input_handler (&mut self, mut input_handler: InputHandler)
    {
        assert!(self.locked == false);

        self.input_handlers.push (input_handler);
    }
}
