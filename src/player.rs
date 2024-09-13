use raylib::prelude::*;
use crate::Object;

pub struct Spaceship {
    pub position: Vector2,
    pub velocity: Vector2,
    pub texture: Texture2D,
    pub last_shot_time: f64,
    pub fire_rate: f64,
}

impl Spaceship {
    pub fn new(thread: &RaylibThread, rl: &mut RaylibHandle) -> Self {
        let position = Vector2 { x: 400.0, y: 550.0 };
        let velocity = Vector2 { x: 0.0, y: 0.0 };
        
        let texture = rl.load_texture(thread, "assets/images/player.png")
            .expect("Não foi possível carregar a textura");

        Spaceship {
            position,
            velocity,
            texture,
            last_shot_time: 0.0,
            fire_rate: 0.5,
        }
    }

    pub fn input(&mut self, rl: &RaylibHandle) {
        let speed = 1.0;

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.velocity.x = speed;
        }
        else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.velocity.x = -speed;
        }
        else {
            self.velocity.x = 0.0;
        }
    }
}

impl Object for Spaceship {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.texture, self.position.x as i32, self.position.y as i32, Color::WHITE);
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.x > 740.0 {
            self.position.x = 740.0;
        }
        
        if self.position.x < 0.0 {
            self.position.x = 0.0;
        }
    }
}
