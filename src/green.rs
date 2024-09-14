use raylib::prelude::*;
use crate::Object;

pub struct Green {
    pub position: Vector2,
    pub velocity: Vector2,
}

pub struct Greens {
    pub greens: Vec<Green>,
    pub texture: Texture2D,
}

impl Greens {
    pub fn new(thread: &RaylibThread, rl: &mut RaylibHandle) -> Self {
        let greens = Vec::new();
        let texture = rl.load_texture(thread, "assets/images/green.png")
            .expect("Não foi possível carregar a textura");

        Greens {
            greens,
            texture,
        }
    }

    pub fn add(&mut self, position: Vector2, velocity: Vector2) {
        let green = Green {
            position,
            velocity,
        };
        self.greens.push(green);
    }

    pub fn check_side(&self) -> bool {
        for green in &self.greens {
            if green.position.x > 760.0 || green.position.x < 0.0 {
                return true;
            }   
        }
        false
    }

    pub fn is_game_over(&self) {
        for green in &self.greens {
            if green.position.y > 530.0 {
                std::process::exit(0);
            }
        }
    }
}

impl Object for Greens {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for green in &self.greens {
            d.draw_texture(&self.texture, green.position.x as i32, green.position.y as i32, Color::WHITE);
        }
    }

    fn update(&mut self) {
        for green in &mut self.greens {
            green.position.x += green.velocity.x;
            green.position.y += green.velocity.y;
        }
    }
}
