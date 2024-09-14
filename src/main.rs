mod player;
mod bullet;
mod green;
mod red;
mod yellow;
mod enemy_bullet;
mod collision;
mod wall;

use raylib::prelude::*;
use player::Spaceship;
use bullet::Bullets;
use green::Greens;
use red::Reds;
use yellow::Yellows;
use enemy_bullet::EnemyBullets;
use collision::Collision;
use wall::Walls;

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
    let mut walls = Walls::new();

    spawn(&mut greens, &mut reds, &mut yellows);
    walls.add(50.0, 530.0);
    walls.add(250.0, 530.0);
    walls.add(450.0, 530.0);
    walls.add(650.0, 530.0);
    
    while !rl.window_should_close() {
        if greens.greens.is_empty() && reds.reds.is_empty() && yellows.yellows.is_empty() {
            spawn(&mut greens, &mut reds, &mut yellows);
        }

        spaceship.input(&rl);
        spaceship.update();
        
        bullets.input(&rl, &mut spaceship);
        bullets.update();

        enemy_bullets.ai(&mut greens, &mut reds, &mut yellows);
        enemy_bullets.update();

        greens.update();
        reds.update();
        yellows.update();

        walls.update();

        if greens.check_side() || reds.check_side() || yellows.check_side() {
            for green in &mut greens.greens {
                green.velocity.x = -green.velocity.x;
                green.position.y += 30.0;
            }
            for red in &mut reds.reds {
                red.velocity.x = -red.velocity.x;
                red.position.y += 30.0;
            }
            for yellow in &mut yellows.yellows {
                yellow.velocity.x = -yellow.velocity.x;
                yellow.position.y += 30.0;
            }
        } 
        
        Collision::enemy_bullet(&mut enemy_bullets, &mut spaceship, &mut walls);
        Collision::player_bullet(&mut bullets, &mut greens, &mut reds, &mut yellows, &mut walls);
        spaceship.is_gameover();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        spaceship.draw(&mut d);
        bullets.draw(&mut d);
        greens.draw(&mut d);
        reds.draw(&mut d);
        yellows.draw(&mut d);
        enemy_bullets.draw(&mut d);
        walls.draw(&mut d);
    }
}

fn spawn(greens: &mut Greens, reds: &mut Reds, yellows: &mut Yellows) {
    let velocity = Vector2 { x: 0.05, y: 0.0 };
    
    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 240.0;
        greens.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 180.0;
        greens.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 120.0;
        reds.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 60.0;
        reds.add(Vector2 { x, y }, velocity);
    }

    for i in 0..11 {
        let x = 60.0 + (i as f32) * 60.0;
        let y = 0.0;
        yellows.add(Vector2 { x, y }, velocity);
    }
}

