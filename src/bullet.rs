use raylib::prelude::*;
use crate::Object;
use crate::player::Spaceship;

struct Bullet {
    position: Vector2,
    velocity: Vector2,
}

pub struct Bullets {
    pub bullets: Vec<Bullet>,
    pub texture: Texture2D,
}

impl Bullets {
    pub fn new(thread: &RaylibThread, rl: &mut RaylibHandle) -> Self {
        let bullets = Vec::new();
        let texture = rl.load_texture(thread, "assets/images/bullet.png")
            .expect("Não foi possível carregar a textura");

        Bullets {
            bullets,
            texture,
        }
    }

    fn shoot(&mut self, position: Vector2, velocity: Vector2) {
        let bullet = Bullet {
            position,
            velocity,
        };
        self.bullets.push(bullet);
    }

    pub fn input(&mut self, rl: &RaylibHandle, spaceship: &mut Spaceship) {
        let time = rl.get_time();

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && (time - spaceship.last_shot_time) >= spaceship.fire_rate {
            let velocity = Vector2 { x: 0.0, y: -1.0 };
            self.shoot(spaceship.position, velocity);

            spaceship.last_shot_time = time;
        }
    }
}

impl Object for Bullets {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for bullet in &self.bullets {
            d.draw_texture(&self.texture, bullet.position.x as i32, bullet.position.y as i32, Color::WHITE);
        }
    }

    fn update(&mut self) {
        for bullet in &mut self.bullets {
            bullet.position.x += bullet.velocity.x;
            bullet.position.y += bullet.velocity.y;
        }

        self.bullets.retain(|bullet| bullet.position.y < 600.0 && bullet.position.y > 0.0);
    }
}
