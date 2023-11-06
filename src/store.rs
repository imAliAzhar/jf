use std::fs::DirEntry;

use crate::actions::Action;

#[derive(Default)]
pub struct State {
    pub counter: i32,
    pub dirs: Vec<DirEntry>,
    pub should_quit: bool,
}

impl State {
    pub fn update(&mut self, msg: Action) {
        match msg {
            Action::Increment => {
                self.counter += 1;
                if self.counter > 50 {
                    self.counter = 0;
                }
            }
            Action::Decrement => {
                self.counter -= 1;
                if self.counter < -50 {
                    self.counter = 0
                }
            }
            Action::Reset => self.counter = 0,
        };
    }

    pub fn add_dirs(&mut self, dirs: Vec<DirEntry>) {
        self.dirs = dirs;
    }
}
