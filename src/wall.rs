use raylib::prelude::*;
use crate::Object;

pub struct Wall {
    pub position: Vector2,
    pub life: i32,
}

pub struct Walls {
    pub walls: Vec<Wall>,
}

impl Walls {
    pub fn new() -> Self {
        Walls {
            walls: Vec::new(),
        }
    }

    pub fn add(&mut self, x: f32, y: f32) {
        let wall = Wall {
            position: Vector2 { x, y },
            life: 24,
        };

        self.walls.push(wall);
    }
}

impl Object for Walls {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for wall in &self.walls {
            d.draw_rectangle(wall.position.x as i32, wall.position.y as i32, 100, 15, Color::GREEN);
        }
    }

    fn update(&mut self) {
        self.walls.retain(|wall| wall.life > 0);
    }
}
