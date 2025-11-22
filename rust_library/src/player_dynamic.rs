use godot::classes::{Sprite2D, ISprite2D, InputEvent, Input};
use godot::global::Key;
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    speed: f64,
    angular_speed: f64,
    base: Base<Sprite2D>,
    keys: Vec<Key>,
    key_states: HashMap<Key, bool>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
            keys: vec![Key::LEFT, Key::RIGHT, Key::UP, Key::DOWN],
            key_states: HashMap::new(),
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32

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

    fn input(&mut self, _event: Gd<InputEvent>) {
        self.keys.iter().for_each(|key| {
            let is_key_pressed = Input::singleton().is_key_pressed(*key);
            self.key_states.insert(*key, is_key_pressed);
        });
    }
}

#[godot_api]
impl Player {
    #[func]
    pub fn increase_speed(&mut self, amount: f64) {
        let old_speed = self.speed;  
        let new_speed = old_speed + amount;
        let speed_ratio: f64;

        if old_speed == 0.0 {
            self.angular_speed = std::f64::consts::PI;
        }
        else {
            speed_ratio = new_speed / old_speed;
            self.angular_speed *= speed_ratio;
        }
        self.speed = new_speed;
        self.speed = self.speed.max(0.0); // Prevent negative speed
    }
    #[signal]
    fn speed_increased();
}
