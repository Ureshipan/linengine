use euclid::default::Point3D;
use macroquad::{color::{self, Color}, rand::gen_range};
use rand::RngExt;


pub struct Line {
    pub p1: Point3D<u32>,
    pub p2: Point3D<u32>,
    color: Color,
}

impl Line {
    pub fn new(point1: Point3D<u32>, point2: Point3D<u32>, in_color: Option<Color>) -> Self {
        let result_color: Color; 
        let mut rng = rand::rng();
        match in_color {
            None => result_color = Color::from_rgba(
                rng.random_range(100..255), 
                rng.random_range(100..255),
                rng.random_range(100..255),
                1),
            Some(in_color) => result_color = in_color,
        }
        return Self { p1: point1, p2: point2, color: result_color };
    }
}