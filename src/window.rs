use crate::callbacks::{CallbackArgs, CallbackTrigger};
use crate::window_config::WindowConfig;
pub use crate::Coloured;
use piston_window::*;
pub use piston_window::{Key, MouseButton};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct Window<T: Coloured + Debug> {
    callbacks: HashMap<CallbackTrigger, Box<dyn FnMut(CallbackArgs) -> Option<Vec<Vec<T>>>>>,
    win: PistonWindow,
    to_draw: Vec<Vec<T>>,
    conf: WindowConfig,
    mouse_pos: (f64, f64),
}

impl<T: Coloured + Debug> Window<T> {
    pub fn new(conf: WindowConfig) -> Self {
        Window {
            callbacks: HashMap::new(),
            win: conf.get_window(),
            to_draw: Vec::new(),
            conf,
            mouse_pos: (0.0, 0.0),
        }
    }
    pub fn set_grid(&mut self, nu: Vec<Vec<T>>) {
        self.to_draw = nu;
    }

    pub fn as_str(&self) -> String {
        format!("Window Conf: {}\nMouse Pos: {:?}\nPlatform Info: {}", self.conf.as_str(), self.mouse_pos, self.win.device.get_info().platform_name.vendor)
    }

    pub fn can_continue<F>(&mut self, mut afterwards: F)
        where
            F: FnMut(&mut Self),
    {
        while let Some(e) = self.win.next() {
            e.mouse_cursor(|pos| {
                self.mouse_pos = arr_to_tuple(pos);
            });

            //region render
            if let Some(r_) = e.render_args() {
                let grid = &self.to_draw;

                self.win.draw_2d(&e, |c, gl, _device| {
                    clear([1.0; 4], gl);

                    if !grid.is_empty() {
                        let (width, height) = match c.viewport.iter().nth(0) {
                            None => {
                                eprintln!("Couldn't get viewport!");
                                (0.0, 0.0)
                            }
                            Some(vp) => arr_to_tuple(vp.window_size),
                        };
                        let cell_width = width / grid[0].len() as f64;
                        let cell_height = height / grid.len() as f64;
                        let rect = [0.0, 0.0, cell_width, cell_height];

                        for x in 0..grid.len() {
                            for y in 0..grid[0].len() {
                                let xpos = y as f64 * cell_width;
                                let ypos = x as f64 * cell_height;
                                let trans = c.transform.trans(xpos, ypos);
                                rectangle(grid[x][y].get_colour(), rect, trans, gl);
                            }
                        }
                    }
                });
            }
            //endregion

            //region callbacks



            let mut new = None;
            'cbs: for (callback, func) in self.callbacks.iter() {
                match callback {
                    CallbackTrigger::Mouse(m, expected_release) => {
                        if let Some(Button::Mouse(btn)) = e.press_args() {
                            if m == &btn && !(*expected_release) {
                                new = func(CallbackArgs::Mouse(self.mouse_pos));
                                break 'cbs;
                            }
                        }
                        if let Some(Button::Mouse(btn)) = e.release_args() {
                            if m == &btn && *expected_release {
                                new = func(CallbackArgs::Mouse(self.mouse_pos));
                                break 'cbs;
                            }
                        }
                    }
                    CallbackTrigger::Scroll => {
                        if let Some(s) = e.mouse_scroll_args() {
                            new = func(CallbackArgs::Scroll(arr_to_tuple(s)));
                            break 'cbs;
                        }
                    }
                    CallbackTrigger::Keyboard(expected_key) => {
                        if let Some(Button::Keyboard(key)) = e.press_args() {
                            if &key == expected_key {
                                new = func(CallbackArgs::None);
                                break 'cbs;
                            }
                        }
                    }
                }
            }

            if let Some(new) = new {
                self.to_draw = new;
            }

            //endregion

            if e.update_args().is_some() {
                afterwards(self);
            }
        }
    }

    pub fn add_callback<F>(mut self, cb: CallbackTrigger, f: &'static F) -> Self
        where
            F: FnMut(CallbackArgs) -> Option<Vec<Vec<T>>>,
    {
        self.callbacks.insert(cb, Box::new(f));
        self
    }
}

fn arr_to_tuple<T: Copy>(a: [T; 2]) -> (T, T) {
    (a[0], a[1])
}