mod callbacks;
mod traits {
    pub trait Coloured {
        fn get_colour(&self) -> [f32; 4];
    }
}
mod window;
mod window_config;

pub use callbacks::*;
pub use traits::Coloured;
pub use window::*;
pub use window_config::WindowConfig;