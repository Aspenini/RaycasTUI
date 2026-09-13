use std::fmt::Write as _;
use std::io::{self, Write};

use crossterm::execute;
use crossterm::terminal::{self, Clear, ClearType};

use crate::map::Map;
use crate::player::Player;
use crate::ray::{self, Side};

#[derive(Debug, Default)]
pub struct Renderer {
    pixels: Vec<u8>,
    output: String,
    last_size: (u16, u16),
    dirty: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            dirty: true,
            ..Self::default()
        }
    }

    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    pub fn draw(&mut self, out: &mut impl Write, map: &Map, player: &Player) -> io::Result<()> {
        let size = terminal::size()?;
        if self.dirty || self.last_size != size {
            execute!(out, Clear(ClearType::All))?;
            self.last_size = size;
            self.dirty = false;
        }

        let width = usize::from(size.0);
        let height = usize::from(size.1);
        if width == 0 || height == 0 {
            return Ok(());
        }

        let double_height = height.saturating_mul(2);
        self.pixels.resize(width * double_height, 0);
        self.trace_columns(width, double_height, map, player);
        self.encode_half_blocks(width, height);

        out.write_all(self.output.as_bytes())?;
        Ok(())
    }

    fn trace_columns(&mut self, width: usize, double_height: usize, map: &Map, player: &Player) {
        let dir = player.dir();
        let plane = player.plane();
        let dh = double_height as f64;

        for x in 0..width {
            let camera_x = (2.0 * x as f64) / width as f64 - 1.0;
            let hit = ray::cast(map, player.pos, dir + plane * camera_x);

            let line_height = dh / hit.dist.max(0.05);
            let center = dh * 0.5;
            let start = ((center - line_height * 0.5).max(0.0) as usize).min(double_height);
            let end = ((center + line_height * 0.5).max(0.0) as usize).min(double_height);
            let wall = wall_color(hit.dist, hit.side);

            for y in 0..start {
                let t = (start - y) as f64 / dh;
                self.pixels[y * width + x] = ceiling_color(t);
            }
            for y in start..end {
                self.pixels[y * width + x] = wall;
            }
            for y in end..double_height {
                let t = (y - end) as f64 / dh;
                self.pixels[y * width + x] = floor_color(t);
            }
        }
    }

    fn encode_half_blocks(&mut self, width: usize, height: usize) {
        self.output.clear();
        let cap = width.saturating_mul(height).saturating_mul(24).max(16);
        if self.output.capacity() < cap {
            self.output.reserve(cap - self.output.capacity());
        }
        self.output.push_str("\x1b[H");

        let mut fg = 0u8;
        let mut bg = 0u8;

        for y in 0..height {
            let upper_row = y * 2 * width;
            let lower_row = (y * 2 + 1) * width;

            for x in 0..width {
                let upper = self.pixels[upper_row + x];
                let lower = self.pixels[lower_row + x];

                if upper != fg || lower != bg {
                    let _ = write!(self.output, "\x1b[38;5;{upper}m\x1b[48;5;{lower}m");
                    fg = upper;
                    bg = lower;
                }
                self.output.push('▀');
            }

            if y + 1 < height {
                self.output.push_str("\x1b[0m\r\n");
                fg = 0;
                bg = 0;
            }
        }

        self.output.push_str("\x1b[0m");
    }
}

fn wall_color(distance: f64, side: Side) -> u8 {
    let dist = distance.clamp(0.1, 15.0);
    let mut t = 1.0 - (dist + 1.0).ln() / 16.0_f64.ln();
    if side == Side::NorthSouth {
        t *= 0.7;
    }
    if t > 0.5 {
        lerp_u8(220, 226, (t - 0.5) * 2.0)
    } else {
        lerp_u8(88, 94, t * 2.0)
    }
}

fn ceiling_color(dist_from_horizon: f64) -> u8 {
    lerp_u8(39, 45, dist_from_horizon)
}

fn floor_color(dist_from_horizon: f64) -> u8 {
    lerp_u8(238, 244, dist_from_horizon)
}

fn lerp_u8(start: u8, end: u8, t: f64) -> u8 {
    let t = t.clamp(0.0, 1.0);
    let start = f64::from(start);
    let end = f64::from(end);
    (start + (end - start) * t).round() as u8
}

#[cfg(test)]
mod tests {
    use super::{ceiling_color, floor_color, lerp_u8, wall_color};
    use crate::ray::Side;

    #[test]
    fn lerp_endpoints() {
        assert_eq!(lerp_u8(10, 20, 0.0), 10);
        assert_eq!(lerp_u8(10, 20, 1.0), 20);
    }

    #[test]
    fn palette_stays_in_256() {
        for dist in [0.1, 1.0, 5.0, 15.0, 100.0] {
            let _ = wall_color(dist, Side::EastWest);
            let _ = wall_color(dist, Side::NorthSouth);
        }
        let _ = ceiling_color(0.0);
        let _ = ceiling_color(2.0);
        let _ = floor_color(0.0);
        let _ = floor_color(2.0);
    }
}
