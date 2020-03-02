use game::CardinalDirection;
use maplit::hashmap;
use prototty::input::KeyboardInput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub enum AppInput {
    Move(CardinalDirection),
    Tech,
    Wait,
}

#[derive(Serialize, Deserialize)]
pub struct Controls {
    keys: HashMap<KeyboardInput, AppInput>,
}

impl Controls {
    pub fn default() -> Self {
        let keys = hashmap![
            KeyboardInput::Left => AppInput::Move(CardinalDirection::West),
            KeyboardInput::Right => AppInput::Move(CardinalDirection::East),
            KeyboardInput::Up => AppInput::Move(CardinalDirection::North),
            KeyboardInput::Down => AppInput::Move(CardinalDirection::South),
            KeyboardInput::Char('t') => AppInput::Tech,
            KeyboardInput::Char(' ') => AppInput::Wait,
        ];
        Self { keys }
    }

    pub fn get(&self, keyboard_input: KeyboardInput) -> Option<AppInput> {
        self.keys.get(&keyboard_input).cloned()
    }
}
