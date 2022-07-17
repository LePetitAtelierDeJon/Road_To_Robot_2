use sfml::graphics::*;
use sfml::system::*;
use std::collections::HashMap;

pub struct Radar {
    pub scan_distance: f32,
    pub echo_radius: f32,
    pub echo_precision: u32,
    pub frame_color: Color,
    pub echo_color: Color,
    pub radius: u16,
    pub frame_nb_point: u16,
    pub position: Vector2f,
    pub detection_angle: u16,
    current_radar_angle: i16,
    echo_map: HashMap<i16, f32>
}

impl Radar {
    pub fn default() -> Self {
        Self {
            position: Vector2f::new(400.0, 600.0),
            radius: 300,
            scan_distance: 100.0,
            detection_angle: 180,
            echo_color: Color::rgb(0, 250, 100),
            frame_color: Color::rgb(250, 250, 250),
            frame_nb_point: 30,
            echo_precision: 4,
            echo_radius: 3.0,
            current_radar_angle: 0,
            echo_map: HashMap::new()
        }
    }
    
    pub fn draw(&self, window: &RenderWindow) {
        self.draw_frame(window);
        self.draw_radar_direction_line(window);
        self.draw_echo(window);
        
    }

    fn draw_frame(&self, window: &RenderWindow) {
        let mut vec = Vec::new();
        let angle_fraction = (self.detection_angle / self.frame_nb_point) as i32;
        vec.push(Vertex::with_pos_color(self.position, self.frame_color));
        for i in 0..(self.frame_nb_point + 1) as i32 {
            let f = -(i * angle_fraction) as f32;
            let x = self.position.x + (self.radius) as f32 * f.to_radians().cos();
            let y = self.position.y + (self.radius) as f32 * f.to_radians().sin();
            let p = Vector2f::new(x, y);
            vec.push(Vertex::with_pos_color(p, self.frame_color));
        }
        vec.push(Vertex::with_pos_color(self.position, self.frame_color));
        window.draw_primitives(&vec[..], PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT);
    }
    
    pub fn draw_radar_direction_line(&self, window: &RenderWindow) {
        let mut vec = Vec::new();
        vec.push(Vertex::with_pos_color(self.position, self.echo_color));
        let f = -(self.current_radar_angle as f32);
        let x = self.position.x + (self.radius) as f32 * f.to_radians().cos();
        let y = self.position.y + (self.radius) as f32 * f.to_radians().sin();
        let p = Vector2f::new(x, y);
        vec.push(Vertex::with_pos_color(p, self.echo_color));
        window.draw_primitives(&vec[..], PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT);
    }

    pub fn draw_echo(&self, window: &RenderWindow) {    
        for (angle, distance) in self.echo_map.iter() {
            let draw_distance = (distance * self.radius as f32) / self.scan_distance;
            let f = -angle;
            let x = self.position.x + draw_distance * (f as f32).to_radians().cos();
            let y = self.position.y + draw_distance * (f as f32).to_radians().sin();
            let point = Vector2f::new(x, y);
            let mut circle = CircleShape::new(self.echo_radius, self.echo_precision);
            circle.set_position(point);
            circle.set_origin(Vector2f::new(self.echo_radius,self.echo_radius));
            circle.set_fill_color(self.echo_color);
            window.draw_circle_shape(&circle, &RenderStates::default());
        }
    }

    pub fn set_current_radar_orientation(&mut self, angle: i16, echo: f32) {
        self.current_radar_angle = angle;
        self.echo_map.insert(angle, echo);
    }
}
