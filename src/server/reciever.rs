use std::{
    io::{Read, ErrorKind},
    net::TcpStream,
};
use super::message::Message;

pub fn recieve_message(mut stream: &TcpStream) -> Result<Message, std::io::Error> {
    let mut buffer = vec![0; 4096]; // Adjust the buffer size as needed
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        // Connection was closed by the client
        return Err(std::io::Error::new(
            ErrorKind::UnexpectedEof,
            "Client disconnected",
        ));
    }

    // Trim the buffer to the actual number of bytes read
    buffer.truncate(bytes_read);

    // Deserialize the message using serde
    let msg = Message::deserialize(&buffer).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Deserialization error: {}", e),
        )
    })?;
    Ok(msg)
}
