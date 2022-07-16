use std::{thread, time};

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

fn draw_frame(window: &RenderWindow) {
    let mut vec = Vec::new();

    let nb_points = 30;
    let angle_max = 180;
    let angle_fraction = angle_max / nb_points;

    vec.push(Vertex::with_pos(Vector2f::new(400.0, 600.0)));

    for i in 0..(nb_points + 1) {
        let f = -(i * angle_fraction) as f32;

        let x = 400.0 + 300.0 * f.to_radians().cos();
        let y = 600.0 + 300.0 * f.to_radians().sin();
        let p = Vector2f::new(x, y);
        vec.push(Vertex::with_pos(p));
    }
    vec.push(Vertex::with_pos_color(
        Vector2f::new(400.0, 600.0),
        Color::rgb(250, 250, 250),
    ));

    window.draw_primitives(&vec[..], PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT);
}

fn draw_line(window: &RenderWindow, angle: i16) {
    let mut vec = Vec::new();

    vec.push(Vertex::with_pos_color(Vector2f::new(400.0, 600.0),
        Color::rgb(0, 250, 100)));
    let f = -(angle as f32);

    let x = 400.0 + 300.0 * f.to_radians().cos();
    let y = 600.0 + 300.0 * f.to_radians().sin();
    let p = Vector2f::new(x, y);
    vec.push(Vertex::with_pos_color(p,
        Color::rgb(0, 250, 100)));

    window.draw_primitives(&vec[..], PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT);
}

fn main() {
    let mut settings = ContextSettings::default().clone();
    settings.set_antialiasing_level(8);

    let mut window = RenderWindow::new((800, 800), "Sonar", Style::CLOSE, &settings);

    let mut angle: i16 = 0;
    let mut orientation: i16 = 1;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        
        window.clear(Color::rgb(30, 30, 30));

        draw_frame(&window);
        draw_line(&window, angle);

        if angle >= 180 {
            orientation = -1;
        } else if angle <= 0 {
            orientation = 1;
        }

        angle += orientation;

        window.display();

        let sleep_duration = time::Duration::from_millis(10);
        thread::sleep(sleep_duration);
    }
}
