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

use std::{thread};

pub struct Game
{
    window: Window,
    sdl_context: Sdl,

    input_handlers: Vec <Box<InputHandler>>,
    locked: bool
}

impl Game {

    pub fn new (name: &str, width: u32, height: u32) -> Result <Game, String>
    {
        let sdl_context = sdl2::init().everything().unwrap();
        let window = match Window::new (&sdl_context, name, width, height) {
            Ok (window) => window,
            Err (err) => return Err (err)
        };

        Ok (Game {
            window: window,
            sdl_context: sdl_context,
            input_handlers: Vec::new(),
            locked: false
        })
    }

    /// For now, every Game-Instance has exactly one Window-Instance, which can be received using
    /// this function.
    pub fn window (&self) -> &Window {
        &self.window
    }

    /// Register a struct that implements the InputHandler-Trait. Whenever input is received, the
    /// handle()-function will be called, which can be used to process any input.
    /// Note that the function will be called from a different thread.
    // TODO: It might be nice to make it possible to still register an event-handler while the loop
    // is running, which requires more sophisticated mechanisms concerning thread-safety.
    pub fn register_input_handler (&mut self, mut input_handler: Box<InputHandler>)
    {
        assert!(self.locked == false);

        self.input_handlers.push (input_handler);
    }

    /// This will start an asynchronous listening process, which will call the handle-function of
    /// of all registered InputHandlers. After calling this, it will be impossible to add any
    /// additional InputHandlers.
    pub fn begin_listen (&mut self) -> bool
    {
        self.locked = true;

        // TODO: Actually spawn a thread and begin to listen here.

    }

    // TODO: Remove this, since the sdl_context should not be directly accessible.
    pub fn context (&mut self) -> &mut Sdl {
        &mut self.sdl_context
    }
}
