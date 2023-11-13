use std::fs::DirEntry;

#[derive(Debug)]
pub enum Action {
    Increment,
    Decrement,
    Reset,
    SetDirs(Vec<DirEntry>),

    Quit,
}
