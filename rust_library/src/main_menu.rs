use godot::classes::{INode};
use godot::prelude::*;
use crate::entry_node::EntryNode;
//use std::hash::Hash;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainMenuNode {
    base: Base<Node>,
}

#[godot_api]
impl INode for MainMenuNode {
    fn init(base: Base<Node>) -> Self {
        return Self { base }
    }
}

#[godot_api]
impl MainMenuNode {

    #[func]
    pub fn new_game_pressed(&mut self) {
        let parent = match self.base().get_parent() {
            Some(p) => p,
            None => {
                godot_error!("Unable to get parent node");
                return
            }
        };
        let mut node: Gd<EntryNode> = match parent.try_cast::<EntryNode>() {
            Err(_) => {
                godot_error!("Parent node is not of type EntryNode");
                return
            },
            Ok(n) => n,
        };
        node.bind_mut().new_game();
    }
}
