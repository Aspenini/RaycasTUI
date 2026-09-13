mod input;
mod map;
mod math;
mod player;
mod ray;
mod render;
mod terminal;

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event};

use crate::input::Input;
use crate::map::WORLD;
use crate::player::Player;
use crate::render::Renderer;
use crate::terminal::Terminal;

fn main() -> io::Result<()> {
    let mut terminal = Terminal::enter()?;
    let mut player = Player::spawn();
    let mut renderer = Renderer::new();
    let mut input = Input::new();
    let mut last = Instant::now();
    const FRAME: Duration = Duration::from_millis(16);

    loop {
        let frame_start = Instant::now();
        let dt = frame_start
            .saturating_duration_since(last)
            .as_secs_f64()
            .min(0.05);
        last = frame_start;

        input.begin_frame();
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(key) => {
                    if input.handle_key(key) {
                        return Ok(());
                    }
                }
                Event::Resize(..) => renderer.invalidate(),
                _ => {}
            }
        }

        player.update(&WORLD, &input, dt);
        renderer.draw(&mut terminal, &WORLD, &player)?;
        terminal.flush()?;

        if let Some(sleep) = FRAME.checked_sub(frame_start.elapsed()) {
            thread::sleep(sleep);
        }
    }
}
