use godot::classes::{Sprite2D, ISprite2D, InputEvent, InputEventMouse};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }

    /*fn input(&mut self, event: Gd<InputEvent>) {
        // Mouse events
        match event.try_cast::<InputEventMouse>() {
            Ok(e) => {
                let mouse_position = e.get_position();
                self.base_mut().set_position(mouse_position);
                return;
            }
            Err(_) => {}
        }
    }*/
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
