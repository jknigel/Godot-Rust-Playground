use godot::prelude::*;
//mod player;
//mod speed_control_buttons;
mod player_steamdeck;
//use player::Player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}