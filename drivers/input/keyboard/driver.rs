#[derive(Default)]
pub struct Keyboard { extended: bool, released: bool }

impl Keyboard {
	pub fn decode_scancode(&mut self, byte: u8) -> Option<KeyEvent> { match byte { 0xe0 => { self.extended = true; None }, 0xf0 => { self.released = true; None }, code => { let event = KeyEvent { code, extended: self.extended, pressed: !self.released }; self.extended = false; self.released = false; Some(event) } } }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyEvent { pub code: u8, pub extended: bool, pub pressed: bool }
