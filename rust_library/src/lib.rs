use godot::prelude::*;
// mod player;
// mod speed_control_buttons;
// mod player_steamdeck;
// mod player_collision;
// mod player_dynamic;
mod player_char2d;
mod entry_node;
mod main_menu;
mod custom_grid;
// use player::Player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}