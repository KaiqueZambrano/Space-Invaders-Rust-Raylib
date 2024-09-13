use raylib::prelude::*;
use crate::Object;

pub struct Red {
    pub position: Vector2,
    pub velocity: Vector2,
}

pub struct Reds {
    pub reds: Vec<Red>,
    pub texture: Texture2D,
}

impl Reds {
    pub fn new(thread: &RaylibThread, rl: &mut RaylibHandle) -> Self {
        let reds = Vec::new();
        let texture = rl.load_texture(thread, "assets/images/red.png")
            .expect("Não foi possível carregar a textura");

        Reds {
            reds,
            texture,
        }
    }

    pub fn add(&mut self, position: Vector2, velocity: Vector2) {
        let red = Red {
            position,
            velocity,
        };
        self.reds.push(red);
    }

    pub fn check_side(&self) -> bool {
        for red in &self.reds {
            if red.position.x > 760.0 || red.position.x < 0.0 {
                return true;
            }   
        }
        false
    }
}

impl Object for Reds {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for red in &self.reds {
            d.draw_texture(&self.texture, red.position.x as i32, red.position.y as i32, Color::WHITE);
        }
    }

    fn update(&mut self) {
        for red in &mut self.reds {
            red.position.x += red.velocity.x;
            red.position.y += red.velocity.y;
        }
    }
}
