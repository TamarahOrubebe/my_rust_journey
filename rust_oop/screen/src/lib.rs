//! We want to create an hypothetical gui tool that can accept any component of any shape and call the draw 
//! method on it to draw it to the screen.

// Trait objects as an OOp concept 
// So we create a Draw trait object to hold a draw method that all components that want to be drawn must implement
pub trait Draw {
    fn draw(&self);
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,

}

impl Draw for Button {
    fn draw(&self) {
        println!("I am drawing a button on the screen")
    }
}


// A screen struct that draws the various components to the screen by iterating over its components field.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}