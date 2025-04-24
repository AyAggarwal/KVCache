use crate::command::state::State;

pub struct Parser {
    state: State,
    buffer: [u8; 1024],
}

impl Parser {
    pub fn new() -> Self {
        Self { state: State::Initial, buffer: [0; 1024] }
    }
}

// Parser needs to take decoded frames and try to advance towards a complete command
// If there is a protocol error the decoder will handle it
// if there is a command error we have to handle it in this command parser