use bytes::BytesMut;
use redis_protocol::resp3::{decode::complete::decode, types::OwnedFrame};

#[derive(Default)]
pub struct Parser {
    current_bytes: BytesMut,
}

impl Parser {
    pub fn new() -> Self { Self::default() }

    pub fn parse_frame(
        &mut self,
        incoming: &[u8],
    ) -> Result<Option<Vec<OwnedFrame>>, Box<dyn std::error::Error>> {
        let mut commands = Vec::new();

        self.current_bytes.reserve(incoming.len());
        // TODO: Zero copy
        self.current_bytes.extend_from_slice(incoming);

        // assume every frame is a command
        loop {
            match decode(&self.current_bytes) {
                Ok(Some((frame, amt))) => {
                    commands.push(frame);
                    _ = self.current_bytes.split_to(amt);
                }
                Ok(None) => {
                    println!("No more frames to parse");
                    break;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        if commands.is_empty() {
            return Ok(None);
        }

        Ok(Some(commands))
    }


}
