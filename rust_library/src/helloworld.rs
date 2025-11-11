
// Define the class first
#[derive(GodotClass)]
#[class(base=Node)]
struct HelloWorld {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for HelloWorld {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        godot_print!("Hello from a correctly built and loaded Rust library!");
    }
}

// Register the class at the end
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}