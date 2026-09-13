use crate::map::{Map, Tile};
use crate::math::Vec2;

const MAX_STEPS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    EastWest,
    NorthSouth,
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub dist: f64,
    pub side: Side,
}

pub fn cast(map: &Map, origin: Vec2, ray: Vec2) -> Hit {
    let delta_x = inv_abs(ray.x);
    let delta_y = inv_abs(ray.y);

    let mut map_x = origin.x.floor() as i32;
    let mut map_y = origin.y.floor() as i32;

    let (step_x, mut side_x) = if ray.x < 0.0 {
        (-1, (origin.x - f64::from(map_x)) * delta_x)
    } else {
        (1, (f64::from(map_x) + 1.0 - origin.x) * delta_x)
    };
    let (step_y, mut side_y) = if ray.y < 0.0 {
        (-1, (origin.y - f64::from(map_y)) * delta_y)
    } else {
        (1, (f64::from(map_y) + 1.0 - origin.y) * delta_y)
    };

    let mut side = Side::EastWest;
    for _ in 0..MAX_STEPS {
        if side_x < side_y {
            side_x += delta_x;
            map_x += step_x;
            side = Side::EastWest;
        } else {
            side_y += delta_y;
            map_y += step_y;
            side = Side::NorthSouth;
        }

        if map.tile(map_x, map_y) == Tile::Wall {
            let dist = match side {
                Side::EastWest => side_x - delta_x,
                Side::NorthSouth => side_y - delta_y,
            };
            return Hit {
                dist: dist.max(1e-6),
                side,
            };
        }
    }

    Hit {
        dist: f64::INFINITY,
        side,
    }
}

fn inv_abs(component: f64) -> f64 {
    if component.abs() < 1e-12 {
        f64::INFINITY
    } else {
        1.0 / component.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::{Side, cast};
    use crate::map::WORLD;
    use crate::math::Vec2;

    #[test]
    fn looking_east_hits_the_mid_wall() {
        let hit = cast(&WORLD, Vec2::new(2.5, 2.5), Vec2::new(1.0, 0.0));
        assert!(hit.dist.is_finite());
        assert!((hit.dist - 9.5).abs() < 0.05, "dist = {}", hit.dist);
        assert_eq!(hit.side, Side::EastWest);
    }

    #[test]
    fn looking_north_hits_the_outer_wall() {
        let hit = cast(&WORLD, Vec2::new(2.5, 2.5), Vec2::new(0.0, -1.0));
        assert!(hit.dist.is_finite());
        assert!((hit.dist - 1.5).abs() < 0.05, "dist = {}", hit.dist);
        assert_eq!(hit.side, Side::NorthSouth);
    }
}
