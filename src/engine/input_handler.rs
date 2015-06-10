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
use sdl2::event::{Event};

use engine::game::{Game};

/// A class that implements this trait and is registered in the Sdl-Context, i.e. the instance of
/// Game that is native to the Program by calling the method register() before(!) beginning the
/// listening-process will be notified whenever an event occurs.
pub trait InputHandler
{
	fn handle (&mut self, event: &Event);
}
