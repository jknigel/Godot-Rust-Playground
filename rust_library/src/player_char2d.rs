use godot::classes::{CharacterBody2D, ICharacterBody2D, Input, InputEvent};
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
        let mut velocity: Vector2 = self.base().get_velocity();
        velocity = self.apply_gravity(velocity);
        velocity = self.apply_controls(velocity);

        self.base_mut().set_velocity(velocity);

        let collided = self.base_mut().move_and_slide();

        if collided {
            godot_print!("Collision detected this frame!");
            self.signals().on_player_did_collide().emit();
        }
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        self.keys.iter().for_each(|key: &Key| {
            let is_key_pressed: bool = Input::singleton().is_key_pressed(*key);
            self.key_states.insert(*key, is_key_pressed);
        });
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn on_player_did_collide();

    fn apply_gravity(&mut self, mut velocity: Vector2) -> Vector2 {
        let gravity: f32 = 9.8; // pixels per second squared
        velocity.y += gravity;
        return velocity;
    }

    fn apply_controls(&mut self, mut velocity: Vector2) -> Vector2 {
        let jump_speed: f32 = 20.0;
        let move_speed: f32 = 250.0;

        velocity.x = 0.0;

        self.keys.iter().for_each(|k| {
            match self.key_states.get(k) {
                Some(true) => match *k {
                    Key::UP => {
                        if self.base().is_on_floor() {
                            velocity.y -= jump_speed;
                        }
                    },
                    Key::LEFT => velocity.x = -move_speed,
                    Key::RIGHT => velocity.x = move_speed,
                    _ => {
                        godot_print!("Key not supported {:?}!", k);
                    }
                },
                _ => return,
            };
        });
        return velocity;
    }
}
