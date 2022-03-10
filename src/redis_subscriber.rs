extern crate redis;
use bincode;
use crate::message::ProverMessage;
use crate::message_handler;
use redis::{ControlFlow, PubSubCommands};
use std::error::Error;

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                // let received: String = msg.get_payload().unwrap();
                // let message_obj = serde_json::from_str::<PubSubMessage>(&received).unwrap();

                let paylaod = msg.get_payload_bytes().unwrap();
                let message_obj = bincode::deserialize(paylaod).unwrap();
                message_handler::handle(message_obj);

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}

pub fn start() {
  // start subscribe for redis
    if let Err(error) = subscribe(String::from("go_channel")) {
        println!("subscribe something was wrong{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }
}


