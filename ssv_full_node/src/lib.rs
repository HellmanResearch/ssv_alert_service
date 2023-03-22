use std::iter::Filter;
use tungstenite::{connect, Message};
use url::Url;


pub struct DecidedFilter {
    from: u32,
    to: u32,
    role: String,
    publicKey: String,
}

pub struct DecidedItemMessage {
    MsgType: u32,
    Height: u32,
    Round: u32,
    Identifier: String,
    Data: String,
}

pub struct DecidedItem {
    Signature: String,
    Signers: Vec<u32>,
    Message: Message,
}


pub struct DecidedResponse {
    filter: DecidedFilter,
    data: Vec<DecidedItem>,
}

fn start_stream() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:15000/stream").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    // println!("Response HTTP code: {}", response.status());
    // println!("Response contains the following headers:");
    // for (ref header, _value) in response.headers() {
    //     println!("* {}", header);
    // }
    //
    // socket.write_message(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
    // socket.close(None);
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_stream();
    }
}
