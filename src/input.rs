use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Clone, Copy, Debug, Default)]
struct Buttons {
    forward: bool,
    back: bool,
    strafe_left: bool,
    strafe_right: bool,
    turn_left: bool,
    turn_right: bool,
}

/// Keyboard state for the current frame.
///
/// Uses press/release tracking once the terminal reports `Release` events
/// (Windows, and Unix terminals with the kitty keyboard protocol). Until then
/// it treats each `Press`/`Repeat` as active for that frame only, so keys do
/// not stick on terminals that never send releases.
#[derive(Debug, Default)]
pub struct Input {
    held: Buttons,
    frame: Buttons,
    use_held: bool,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_frame(&mut self) {
        self.frame = Buttons::default();
    }

    /// Returns `true` when the user asked to quit.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if is_quit(&key) && key.kind != KeyEventKind::Release {
            return true;
        }

        let Some(slot) = binding(key.code) else {
            return false;
        };

        match key.kind {
            KeyEventKind::Release => {
                self.use_held = true;
                *slot.field(&mut self.held) = false;
            }
            KeyEventKind::Press => {
                *slot.field(&mut self.held) = true;
                *slot.field(&mut self.frame) = true;
            }
            KeyEventKind::Repeat => {
                *slot.field(&mut self.frame) = true;
            }
        }

        false
    }

    fn buttons(&self) -> Buttons {
        if self.use_held { self.held } else { self.frame }
    }

    pub fn forward(&self) -> bool {
        self.buttons().forward
    }

    pub fn back(&self) -> bool {
        self.buttons().back
    }

    pub fn strafe_left(&self) -> bool {
        self.buttons().strafe_left
    }

    pub fn strafe_right(&self) -> bool {
        self.buttons().strafe_right
    }

    /// `-1` left, `+1` right, `0` if both or neither.
    pub fn turn(&self) -> f64 {
        let buttons = self.buttons();
        f64::from(i8::from(buttons.turn_right) - i8::from(buttons.turn_left))
    }
}

#[derive(Clone, Copy)]
enum Binding {
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    TurnLeft,
    TurnRight,
}

impl Binding {
    fn field(self, buttons: &mut Buttons) -> &mut bool {
        match self {
            Self::Forward => &mut buttons.forward,
            Self::Back => &mut buttons.back,
            Self::StrafeLeft => &mut buttons.strafe_left,
            Self::StrafeRight => &mut buttons.strafe_right,
            Self::TurnLeft => &mut buttons.turn_left,
            Self::TurnRight => &mut buttons.turn_right,
        }
    }
}

fn binding(code: KeyCode) -> Option<Binding> {
    Some(match code {
        KeyCode::Char('w' | 'W') | KeyCode::Up => Binding::Forward,
        KeyCode::Char('s' | 'S') | KeyCode::Down => Binding::Back,
        KeyCode::Char('a' | 'A') => Binding::StrafeLeft,
        KeyCode::Char('d' | 'D') => Binding::StrafeRight,
        KeyCode::Left => Binding::TurnLeft,
        KeyCode::Right => Binding::TurnRight,
        _ => return None,
    })
}

fn is_quit(key: &KeyEvent) -> bool {
    matches!(key.code, KeyCode::Esc | KeyCode::Char('q' | 'Q'))
        || (key.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key.code, KeyCode::Char('c' | 'C')))
}

#[cfg(test)]
mod tests {
    use super::{Input, is_quit};
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn key(code: KeyCode, kind: KeyEventKind) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn quit_keys() {
        let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let q = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(is_quit(&esc));
        assert!(is_quit(&q));
        assert!(is_quit(&ctrl_c));
    }

    #[test]
    fn press_moves_this_frame_without_release() {
        let mut input = Input::new();
        input.begin_frame();
        assert!(!input.handle_key(key(KeyCode::Char('w'), KeyEventKind::Press)));
        assert!(input.forward());

        input.begin_frame();
        assert!(!input.forward());
    }

    #[test]
    fn release_enables_held_tracking() {
        let mut input = Input::new();
        input.begin_frame();
        input.handle_key(key(KeyCode::Char('w'), KeyEventKind::Press));
        input.handle_key(key(KeyCode::Char('w'), KeyEventKind::Release));

        input.begin_frame();
        input.handle_key(key(KeyCode::Char('d'), KeyEventKind::Press));
        assert!(input.strafe_right());

        input.begin_frame();
        assert!(input.strafe_right());
        assert!(!input.forward());
    }
}
