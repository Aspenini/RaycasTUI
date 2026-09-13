use std::io::{self, Stdout, Write, stdout};

use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{
    self, DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen,
};

/// Alternate-screen session that restores the terminal on drop, including
/// panic unwind.
pub struct Terminal {
    stdout: Stdout,
    restored: bool,
}

impl Terminal {
    pub fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        if let Err(err) = execute!(stdout, EnterAlternateScreen, Hide, DisableLineWrap) {
            let _ = terminal::disable_raw_mode();
            return Err(err);
        }
        Ok(Self {
            stdout,
            restored: false,
        })
    }

    fn restore(&mut self) -> io::Result<()> {
        if self.restored {
            return Ok(());
        }
        execute!(self.stdout, Show, EnableLineWrap, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        self.stdout.flush()?;
        self.restored = true;
        Ok(())
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
