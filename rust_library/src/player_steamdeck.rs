use godot::classes::{Sprite2D, ISprite2D, InputEvent, InputEventMouse, Input};
use godot::global::Key;
use godot::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {

    base: Base<Sprite2D>,
    keys: Vec<Key>,
    key_states: HashMap<Key, bool>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            base,
            keys: vec![Key::LEFT, Key::RIGHT, Key::UP, Key::DOWN],
            key_states: HashMap::new(),
        }
    }

    fn physics_process(&mut self, delta: f64) {

        let mut current_position = self.base_mut().get_position();

        let keys_to_check: Vec<Key> = self.keys.clone();
        for k in keys_to_check {
            match self.key_states.get(&k) {
                Some(true) => {},
                _ => continue
            };

            match k {
                Key::UP => { current_position.y -= 1.0 * 1000.0 * delta as f32; },
                Key::DOWN => { current_position.y += 1.0 * 1000.0 * delta as f32; },
                Key::LEFT => { current_position.x -= 1.0 * 1000.0 * delta as f32; },
                Key::RIGHT => {current_position.x += 1.0 * 1000.0 * delta as f32; },
                _ => {
                    godot_print!("Key not supported {:?}!", k);
                }
            }
            self.base_mut().set_position(current_position);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.keys.iter().for_each(|key| {
            let is_key_pressed = Input::singleton().is_key_pressed(*key);
            self.key_states.insert(*key, is_key_pressed);
        });
    }
}
