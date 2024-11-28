use super::handler::Handler;
use super::message::Message;
use std::sync::Arc;

pub fn send_message(
    handler: Arc<Handler>,
    client_id: usize,
    msg: &Message,
) -> Result<(), std::io::Error> {
    handler.send_message(client_id, msg)
}
