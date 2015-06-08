extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};


fn main ()
{
    let mut context = sdl2::init().everything().unwrap();

    let window = match context.window("Rust SDL", 800, 600).position_centered().opengl().build() {
        Ok (window) => window,
        Err (err) => panic!("Unable to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
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

    let mut events = context.event_pump();

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
