use godot::prelude::*;
use godot::classes::{GridContainer, IGridContainer, Label, Node};

#[derive(GodotClass)]
#[class(tool, base=GridContainer)]
pub struct CustomGrid {
    base: Base<GridContainer>,
}

#[godot_api]
impl IGridContainer for CustomGrid {
    fn init(base: Base<GridContainer>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        self.init_children();
    }
}

#[godot_api]
impl CustomGrid {
    #[func]
    fn init_children(&mut self) {
        let mut base_mut = self.base_mut();
        for x in 0..10 {

            let mut label = Label::new_alloc();

            let rust_string = format!("Item {}", x);
            label.set_text(&rust_string);
            
            let node: Gd<Node> = label.upcast();
            base_mut.add_child(&node);
        }
    }
}