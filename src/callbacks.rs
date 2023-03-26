use piston_window::{Key, MouseButton};

#[derive(Copy, Clone, Debug)]
pub enum CallbackArgs {
    Mouse((f64, f64)),
    Scroll((f64, f64)),
    None,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CallbackTrigger {
    Mouse(MouseButton, bool),
    Scroll,
    Keyboard(Key),
}