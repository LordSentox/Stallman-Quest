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
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};

pub mod engine;

use engine::window::{Window};
use engine::game::{Game};

fn main ()
{
    let mut game = match Game::new("Stallman-Quest", 800, 600) {
        Ok (game) => game,
        Err (err) => panic!("Unable to create game instance: {}", err)
    };

    let mut renderer = match game.window().sdl_window.renderer().build() {
        Ok (renderer) => renderer,
        Err (err) => panic!("Unable to create renderer: {}", err)
    };

    let mut drawer = renderer.drawer();
    let _ = drawer.set_draw_color (Color::RGB(15, 15, 15));
    let _ = drawer.clear();
    let _ = drawer.set_draw_color (Color::RGB(250, 250, 250));

    let inner_rect = Rect::new (10, 10, 300, 200);
    let _ = drawer.fill_rect (inner_rect);

    let _ = drawer.present();

    let mut events = game.window().sdl_context.event_pump();

    loop
    {
        for event in events.poll_iter()
        {
            match event {
                Event::Quit{..} => return,
                _ => continue
            }
        }
    };
}
