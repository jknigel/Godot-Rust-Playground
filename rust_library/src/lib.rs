use godot::prelude::*;
mod player;
mod speed_control_buttons;
//use player::Player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}