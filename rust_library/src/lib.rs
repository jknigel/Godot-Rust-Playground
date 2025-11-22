use godot::prelude::*;
// mod player;
// mod speed_control_buttons;
// mod player_steamdeck;
// mod player_collision;
mod player_dynamic;
mod custom_grid;
// use player::Player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}