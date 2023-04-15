use std::net::TcpListener;
use std::thread::spawn;
use std::{thread, time};
use tungstenite::accept;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

use std::sync::mpsc;

mod radar_file;
use radar_file::Radar;

fn main() {
    let mut settings = ContextSettings::default().clone();
    settings.set_antialiasing_level(8);

    let mut window = RenderWindow::new((800, 800), "Radar", Style::CLOSE, &settings);

    let mut radar = Radar::default();

    let font = Font::from_file("./ZabalDEMO-Bold.otf").unwrap();

    let mut title = Text::new("SONAR", &font, 90);
    let bounds = title.local_bounds();
    title.set_origin(Vector2f::new(bounds.width / 2.0, bounds.height / 2.0));
    title.set_position(Vector2f::new(400.0, 100.0));

    let mut title2 = Text::new("by LePetitAtelierDeJon", &font, 30);
    let bounds = title2.local_bounds();
    title2.set_origin(Vector2f::new(bounds.width / 2.0, bounds.height / 2.0));
    title2.set_position(Vector2f::new(400.0, 180.0));

    let (tx, rx) = mpsc::channel();
    let (thread_end_tx, thread_end_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let server = TcpListener::bind("localhost:3012").unwrap();
        for stream in server.incoming() {
            let tx_ter = tx.clone();
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                loop {
                    let msg = websocket.read_message().unwrap();
                    let content = msg.to_text().unwrap();
                    tx_ter.send(content.to_owned()).unwrap();
                }
            });
        }
    });

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }        

        window.clear(Color::rgb(30, 30, 30));

        match rx.try_recv() {
            Ok(resp) => {
                let mut splited_message = resp.split(";");
                let str_angle = splited_message.next().unwrap();
                let angle: i16 = str_angle.parse().unwrap();
                let str_echo = splited_message.next().unwrap();
                let echo: f32 = str_echo.parse().unwrap();

                radar.set_current_radar_orientation(angle, echo);
            }
            Err(_) => {}
        }

        radar.draw(&window);
        window.draw_text(&title, &RenderStates::DEFAULT);
        window.draw_text(&title2, &RenderStates::DEFAULT);
        window.display();

        let sleep_duration = time::Duration::from_millis(10);
        thread::sleep(sleep_duration);
    }
    thread_end_tx.send(());
}
