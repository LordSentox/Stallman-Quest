extern crate sfml;

use sfml::window::{ContextSettings, VideoMode, event, WindowStyle};
use sfml::graphics::{RenderWindow, RenderTarget, Color, Text, Font};

fn main ()
{
    let mut window =
    match RenderWindow::new (VideoMode::new_init (800, 600, 32), "Rust-SFML", WindowStyle::Close, &ContextSettings::default()) {
        Some (window) => window,
        None => panic!("Cannot create a new RenderWindow!")
    };

    let vegur = match Font::new_from_file ("Vegur-Regular.otf") {
        Some (vegur) => vegur,
        None => panic!("Cannot open 'Vegur-Regular.otf'")
    };

    let mut greeting = match Text::new () {
        Some (greeting) => greeting,
        None => panic!("Cannot create sfml-text")
    };

    greeting.set_string ("Hello World");
    greeting.set_font (&vegur);

    while window.is_open()
    {
        for event in window.events()
        {
            match event
            {
                event::Closed => window.close(),
                _ => {}
            }
        }

        window.clear (&Color::new_rgb (43, 37, 37));

        window.draw (&greeting);

        window.display();
    }
}
