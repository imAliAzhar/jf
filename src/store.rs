use crate::message::Message;

#[derive(Default)]
pub struct Model {
    pub counter: i32,
    pub should_quit: bool,
}

impl Model {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Increment => {
                self.counter += 1;
                if self.counter > 50 {
                    self.counter = 0;
                }
            }
            Message::Decrement => {
                self.counter -= 1;
                if self.counter < -50 {
                    self.counter = 0
                }
            }
            Message::Reset => self.counter = 0,
            Message::Quit => self.should_quit = true, // You can handle cleanup and exit here
        };
    }
}
