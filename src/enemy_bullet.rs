use raylib::prelude::*;
use crate::Object;
use crate::green::Greens;
use crate::red::Reds;
use crate::yellow::Yellows;
use rand::Rng;

pub struct EnemyBullet {
    pub position: Vector2,
    pub velocity: Vector2,
}

pub struct EnemyBullets {
    pub bullets: Vec<EnemyBullet>,
    pub texture: Texture2D,
}

impl EnemyBullets {
    pub fn new(thread: &RaylibThread, rl: &mut RaylibHandle) -> Self {
        let bullets = Vec::new();
        let texture = rl.load_texture(thread, "assets/images/enemy_bullet.png")
            .expect("Não foi possível carregar a textura");

        EnemyBullets {
            bullets,
            texture,
        }
    }

    fn shoot(&mut self, position: Vector2, velocity: Vector2) {
        let enemy_bullet = EnemyBullet {
            position,
            velocity,
        };
        self.bullets.push(enemy_bullet);
    }


    pub fn ai(&mut self,
              the_greens: &mut Greens,
              the_reds: &mut Reds,
              the_yellows: &mut Yellows) {

        let mut rng = rand::thread_rng();

        let shoot_probability = 0.1;

        if !the_greens.greens.is_empty() {
            if rng.gen_range(0.0 .. 100.0) < shoot_probability {
                let i = rng.gen_range(0..the_greens.greens.len());
                let velocity = Vector2 { x: 0.0, y: 0.7 };
                self.shoot(the_greens.greens[i].position, velocity);
            }
        }
        else if !the_reds.reds.is_empty() {
            if rng.gen_range(0.0 .. 100.0) < shoot_probability {
                let i = rng.gen_range(0..the_reds.reds.len());
                let velocity = Vector2 { x: 0.0, y: 0.7 };
                self.shoot(the_reds.reds[i].position, velocity);
            }
        }
        else if !the_yellows.yellows.is_empty() {
            if rng.gen_range(0.0 .. 100.0) < shoot_probability {
                let i = rng.gen_range(0..the_yellows.yellows.len());
                let velocity = Vector2 { x: 0.0, y: 0.7 };
                self.shoot(the_yellows.yellows[i].position, velocity);
            }
        }
    }
}


impl Object for EnemyBullets {
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
