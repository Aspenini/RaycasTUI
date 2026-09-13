use std::f64::consts::TAU;

use crate::input::Input;
use crate::map::Map;
use crate::math::Vec2;

const MOVE_SPEED: f64 = 3.0;
const ROT_SPEED: f64 = 2.0;
const COLLISION_RADIUS: f64 = 0.15;
const FOV_PLANE: f64 = 0.66;

#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub pos: Vec2,
    angle: f64,
}

impl Player {
    pub fn spawn() -> Self {
        Self {
            pos: Vec2::new(2.5, 2.5),
            angle: 0.0,
        }
    }

    pub fn dir(&self) -> Vec2 {
        Vec2::from_angle(self.angle)
    }

    pub fn plane(&self) -> Vec2 {
        self.dir().perp_right() * FOV_PLANE
    }

    pub fn update(&mut self, map: &Map, input: &Input, dt: f64) {
        self.angle = (self.angle + input.turn() * ROT_SPEED * dt).rem_euclid(TAU);

        let dir = self.dir();
        let mut wish = Vec2::ZERO;
        if input.forward() {
            wish += dir;
        }
        if input.back() {
            wish -= dir;
        }
        if input.strafe_left() {
            wish += dir.strafe_left();
        }
        if input.strafe_right() {
            wish += dir.perp_right();
        }

        if let Some(wish) = wish.normalized() {
            self.try_move(map, wish * (MOVE_SPEED * dt));
        }
    }

    fn try_move(&mut self, map: &Map, delta: Vec2) {
        let nx = self.pos.x + delta.x;
        let ny = self.pos.y + delta.y;
        if map.walkable(nx, self.pos.y, COLLISION_RADIUS) {
            self.pos.x = nx;
        }
        if map.walkable(self.pos.x, ny, COLLISION_RADIUS) {
            self.pos.y = ny;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{COLLISION_RADIUS, Player};
    use crate::input::Input;
    use crate::map::WORLD;
    use crate::math::Vec2;

    #[test]
    fn spawn_is_walkable() {
        let player = Player::spawn();
        assert!(WORLD.walkable(player.pos.x, player.pos.y, COLLISION_RADIUS));
    }

    #[test]
    fn slides_along_walls() {
        let mut player = Player {
            pos: Vec2::new(2.5, 1.2),
            angle: 0.0,
        };
        player.try_move(&WORLD, Vec2::new(0.0, -1.0));
        assert!((player.pos.y - 1.2).abs() < 1e-12);

        let x_before = player.pos.x;
        player.try_move(&WORLD, Vec2::new(0.4, 0.0));
        assert!(player.pos.x > x_before);
    }

    #[test]
    fn idle_update_does_not_move() {
        let mut player = Player::spawn();
        let start = player.pos;
        player.update(&WORLD, &Input::new(), 1.0 / 60.0);
        assert_eq!(player.pos, start);
    }
}
