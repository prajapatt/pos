#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameAction { Move, Aim, Jump, Fire, Pause }

#[derive(Default)]
pub struct InputMapper { bindings: Vec<(u16, GameAction)> }
impl InputMapper { pub fn bind(&mut self, button: u16, action: GameAction) { self.bindings.push((button, action)); } pub fn action_for(&self, button: u16) -> Option<GameAction> { self.bindings.iter().find(|(candidate, _)| *candidate == button).map(|(_, action)| *action) } }
