use raylib::prelude::*;
use crate::Object;

pub struct Yellow {
    pub position: Vector2,
    pub velocity: Vector2,
}

pub struct Yellows {
    pub yellows: Vec<Yellow>,
    pub texture: Texture2D,
}

impl Yellows {
    pub fn new(thread: &RaylibThread, rl: &mut RaylibHandle) -> Self {
        let yellows = Vec::new();
        let texture = rl.load_texture(thread, "assets/images/yellow.png")
            .expect("Não foi possível carregar a textura");

        Yellows {
            yellows,
            texture,
        }
    }

    pub fn add(&mut self, position: Vector2, velocity: Vector2) {
        let yellow = Yellow {
            position,
            velocity,
        };
        self.yellows.push(yellow);
    }

    pub fn check_side(&self) -> bool {
        for yellow in &self.yellows {
            if yellow.position.x > 760.0 || yellow.position.x < 0.0 {
                return true;
            }   
        }
        false
    }
}

impl Object for Yellows {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for yellow in &self.yellows {
            d.draw_texture(&self.texture, yellow.position.x as i32, yellow.position.y as i32, Color::WHITE);
        }
    }

    fn update(&mut self) {
        for yellow in &mut self.yellows {
            yellow.position.x += yellow.velocity.x;
            yellow.position.y += yellow.velocity.y;
        }
    }
}
