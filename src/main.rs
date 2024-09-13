mod player;
mod bullet;
mod green;
mod red;
mod yellow;
mod enemy_bullet;

use raylib::prelude::*;
use player::Spaceship;
use bullet::Bullets;
use green::Greens;
use red::Reds;
use yellow::Yellows;
use enemy_bullet::EnemyBullets;

trait Object {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("SPACE INVADERS")
        .build();

    let mut spaceship = Spaceship::new(&thread, &mut rl);
    let mut bullets = Bullets::new(&thread, &mut rl);
    let mut greens = Greens::new(&thread, &mut rl);
    let mut reds = Reds::new(&thread, &mut rl);
    let mut yellows = Yellows::new(&thread, &mut rl);
    let mut enemy_bullets = EnemyBullets::new(&thread, &mut rl);

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 240.0;
        let velocity = Vector2 { x: 0.1, y: 0.0 };
        greens.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 180.0;
        let velocity = Vector2 { x: 0.1, y: 0.0 };
        greens.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 120.0;
        let velocity = Vector2 { x: 0.1, y: 0.0 };
        reds.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 60.0;
        let velocity = Vector2 { x: 0.1, y: 0.0 };
        reds.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 0.0;
        let velocity = Vector2 { x: 0.1, y: 0.0 };
        yellows.add(Vector2 { x, y }, velocity);
    }
    
    while !rl.window_should_close() {
        spaceship.input(&rl);
        spaceship.update();
        
        bullets.input(&rl, &mut spaceship);
        bullets.update();

        enemy_bullets.ai(&rl, &mut greens, &mut reds, &mut yellows);
        enemy_bullets.update();

        greens.update();
        reds.update();
        yellows.update();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        spaceship.draw(&mut d);
        bullets.draw(&mut d);
        greens.draw(&mut d);
        reds.draw(&mut d);
        yellows.draw(&mut d);
        enemy_bullets.draw(&mut d);
    }
}
