use std::{thread, time};

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

mod radar_file;
use radar_file::Radar;

fn main() {
    let mut settings = ContextSettings::default().clone();
    settings.set_antialiasing_level(8);

    let mut window = RenderWindow::new((800, 800), "Radar", Style::CLOSE, &settings);

    let mut radar = Radar::default();

    let mut angle: i16 = 0;
    let mut orientation: i16 = 1;

    let font = Font::from_file("./PIXEAB__.TTF").unwrap();

    let mut title = Text::new("SONAR", &font, 60);
    let bounds = title.local_bounds();
    title.set_origin(Vector2f::new(bounds.width/2.0, bounds.height/2.0));
    title.set_position(Vector2f::new(400.0, 100.0));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }        

        window.clear(Color::rgb(30, 30, 30));

        if angle >= 180 {
            orientation = -1;
        } else if angle <= 0 {
            orientation = 1;
        }

        angle += orientation;
       
        radar.set_current_radar_orientation(angle, -1.0);        

        radar.draw(&window);
        window.draw_text(&title, &RenderStates::DEFAULT);
        window.display();

        let sleep_duration = time::Duration::from_millis(10);
        thread::sleep(sleep_duration);
    }
}
