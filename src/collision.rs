use raylib::prelude::*;
use crate::bullet::Bullets;
use crate::enemy_bullet::EnemyBullets;
use crate::player::Spaceship;
use crate::green::Greens;
use crate::red::Reds;
use crate::yellow::Yellows;

pub struct Collision;

impl Collision {
    pub fn enemy_bullet(the_bullet: &mut EnemyBullets, player: &mut Spaceship) {
        the_bullet.bullets.retain(|bullet| {
            let rec1 = Rectangle {
                x: bullet.position.x,
                y: bullet.position.y,
                width: the_bullet.texture.width() as f32,
                height: the_bullet.texture.height() as f32,
            };

            let rec2 = Rectangle {
                x: player.position.x,
                y: player.position.y,
                width: player.texture.width() as f32,
                height: player.texture.height() as f32,
            };

            if rec1.check_collision_recs(&rec2) {
                player.life -= 1;
                return false;
            }

            true
        });
    }

    pub fn player_bullet(
        the_bullets: &mut Bullets, 
        the_greens: &mut Greens,
        the_reds: &mut Reds,
        the_yellows: &mut Yellows,
    ) {
        the_bullets.bullets.retain(|bullet| {
            let mut hit_enemy = false;

            the_greens.greens.retain(|enemy| {
                let rec1 = Rectangle {
                    x: bullet.position.x,
                    y: bullet.position.y,
                    width: the_bullets.texture.width() as f32 /2.0,
                    height: the_bullets.texture.height() as f32 /2.0,
                };

                let rec2 = Rectangle {
                    x: enemy.position.x,
                    y: enemy.position.y,
                    width: the_greens.texture.width() as f32,
                    height: the_greens.texture.height() as f32,
                };

                if rec1.check_collision_recs(&rec2) {
                    hit_enemy = true;
                    return false;
                }

                true
            });

            the_reds.reds.retain(|enemy| {
                let rec1 = Rectangle {
                    x: bullet.position.x,
                    y: bullet.position.y,
                    width: the_bullets.texture.width() as f32 /2.0,
                    height: the_bullets.texture.height() as f32 /2.0,
                };

                let rec2 = Rectangle {
                    x: enemy.position.x,
                    y: enemy.position.y,
                    width: the_reds.texture.width() as f32,
                    height: the_reds.texture.height() as f32,
                };

                if rec1.check_collision_recs(&rec2) {
                    hit_enemy = true;
                    return false;
                }

                true
            });

            the_yellows.yellows.retain(|enemy| {
                let rec1 = Rectangle {
                    x: bullet.position.x,
                    y: bullet.position.y,
                    width: the_bullets.texture.width() as f32 /2.0,
                    height: the_bullets.texture.height() as f32 /2.0,
                };

                let rec2 = Rectangle {
                    x: enemy.position.x,
                    y: enemy.position.y,
                    width: the_yellows.texture.width() as f32,
                    height: the_yellows.texture.height() as f32,
                };

                if rec1.check_collision_recs(&rec2) {
                    hit_enemy = true;
                    return false;
                }

                true
            });

            !hit_enemy
        });
    }
}
