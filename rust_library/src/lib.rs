use godot::prelude::*;
mod player;
//use player::Player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}