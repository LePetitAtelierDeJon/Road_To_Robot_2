// use std::{thread, time};

use sfml::graphics::*;
// use sfml::system::*;
use sfml::window::*;

mod radar_file;
use radar_file::Radar;

fn main() {
    let mut settings = ContextSettings::default().clone();
    settings.set_antialiasing_level(8);

    let mut window = RenderWindow::new((800, 800), "Radar", Style::CLOSE, &settings);

    let mut radar = Radar::default();

    // let mut angle: i16 = 0;
    // let mut orientation: i16 = 1;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }        

        window.clear(Color::rgb(30, 30, 30));

        radar.set_current_radar_orientation(45, 50.0);

        // radar.draw(&window);
        // radar.draw_radar_direction_line(&window, angle);
        // radar.draw_echo(&window, obstacle_point);

        // if angle >= 180 {
        //     orientation = -1;
        // } else if angle <= 0 {
        //     orientation = 1;
        // }

        // angle += orientation;

        // window.display();

        radar.set_current_radar_orientation(1, 40.0);
        radar.set_current_radar_orientation(125, 90.0);
        radar.set_current_radar_orientation(145, 95.0);
        radar.set_current_radar_orientation(179, 1.0);
        radar.set_current_radar_orientation(0, 100.0);

        radar.draw(&window);
        window.display();

        // let sleep_duration = time::Duration::from_millis(10);
        // thread::sleep(sleep_duration);
    }
}
