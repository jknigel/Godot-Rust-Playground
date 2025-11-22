use godot::classes::{ICharacterBody2D, CharacterBody2D, InputEvent, Input};
use godot::global::Key;
use godot::prelude::*;
use std::collections::HashMap;
//use std::hash::Hash;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {

    base: Base<CharacterBody2D>,
    keys: Vec<Key>,
    key_states: HashMap<Key, bool>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            base,
            keys: vec![Key::LEFT, Key::RIGHT, Key::UP],
            key_states: HashMap::new(),
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let mut velocity: Vector2 = self.base_mut().get_velocity();
        velocity = self.apply_gravity(velocity);
        velocity = self.apply_controls(velocity);
        
        let mut base = self.base_mut();
        if let Some(c) = base.move_and_collide(velocity) {
            velocity = velocity.slide(c.get_normal());
        }
        base.move_and_collide(velocity);
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        self.keys.iter().for_each(|key: &Key| {
            let is_key_pressed: bool = Input::singleton().is_key_pressed(*key);
            self.key_states.insert(*key, is_key_pressed);
        });
    }
}

impl Player {
    fn apply_gravity(&mut self, mut velocity: Vector2) -> Vector2 {
        let gravity: f32 = 9.8; // pixels per second squared
        velocity.y += gravity;
        return velocity

    }

    fn apply_controls(&mut self, mut velocity: Vector2) -> Vector2 {
        let jump_speed: f32 = 20.0;
        let move_speed: f32 = 10.0;

        self.keys.iter().for_each(|k| {
            match self.key_states.get(&k) {
                Some(true) => {},
                _ => return,
            };

            match *k {
                Key::UP => velocity.y -= jump_speed,
                Key::LEFT => velocity.x -= 1.0 * move_speed,
                Key::RIGHT => velocity.x += 1.0 * move_speed,
                _ => {
                    godot_print!("Key not supported {:?}!", k);
                }
            }
        });
        return velocity
    } 
}
