// src/speed_control_button.rs

use godot::prelude::*;
use godot::classes::{Button, IButton};
use crate::player::Player;

#[derive(GodotConvert, Var, Export, PartialEq, Clone, Copy)]
#[godot(via = GString)]
pub enum SpeedOperation {
    Increase,
    Decrease,
}

#[derive(GodotClass)]
#[class(base=Button)]
pub struct SpeedControlButton {
    #[export]
    pub operation: SpeedOperation,
    #[export]
    pub amount: f64,
    #[base]
    base: Base<Button>,
}

#[godot_api]
impl IButton for SpeedControlButton {
    fn init(base: Base<Button>) -> Self { 
        Self { 
            operation: SpeedOperation::Increase, 
            amount: 100.0, 
            base 
        } 
    }
    
    fn ready(&mut self) {
        let myself: Gd<SpeedControlButton> = self.to_gd();
        let mut button: Gd<Button> = myself.clone().upcast();

        // --- THE FINAL, CORRECTED CONNECTION ---
        // 1. Use the string literal directly.
        // 2. Pass a reference (&) to the callable.
        button.connect("pressed", &myself.callable("on_pressed"));
    }
}

#[godot_api]
impl SpeedControlButton {
    #[func]
    fn on_pressed(&mut self) {
        // Use "players" directly without .into()
        let maybe_node = self.base().get_tree().unwrap().get_first_node_in_group("players");

        if let Some(node) = maybe_node {
            if let Ok(mut player) = node.try_cast::<Player>() {
                match self.operation {
                    SpeedOperation::Increase => {
                        player.bind_mut().increase_speed(self.amount);
                    }
                    SpeedOperation::Decrease => {
                        player.bind_mut().increase_speed(-self.amount);
                    }
                }
            } else {
                godot_warn!("Found a node in 'players' group, but it could not be cast to Player.");
            }
        } else {
            godot_warn!("SpeedControlButton could not find any node in the 'players' group!");
        }
    }
}