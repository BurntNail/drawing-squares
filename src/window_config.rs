use piston_window::{PistonWindow, WindowSettings};

pub struct WindowConfig {
    win_name: String,
    win_size: [u32; 2],
    exit_on_esc: bool,
    resizable: bool,
}

impl WindowConfig {
    pub fn default() -> Self {
        WindowConfig {
            win_name: "Hello World!".to_string(),
            win_size: [400, 400],
            exit_on_esc: true,
            resizable: true,
        }
    }
    pub fn new(win_name: String, win_size: [u32; 2], exit_on_esc: bool, resizable: bool) -> Self {
        WindowConfig {
            win_name,
            win_size,
            exit_on_esc,
            resizable,
        }
    }
    pub(crate) fn get_window(&self) -> PistonWindow {
        let win: PistonWindow = WindowSettings::new(self.win_name.as_str(), self.win_size)
            .exit_on_esc(self.exit_on_esc)
            .resizable(self.resizable)
            .build()
            .unwrap_or_else(|e| {
                eprintln!("ERROR MAKING WINDOW: {}", e);
                std::process::exit(1);
            });
        win
    }
    pub fn as_str (&self) -> String {
        format!("NAME: {}, SIZE: {:?}, EXIT_ON_ESC: {}, RESIZE: {}", self.win_name, self.win_size, self.exit_on_esc, self.resizable)
    }
}