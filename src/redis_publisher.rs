extern crate redis;
use bincode;
use crate::message::ProverMessage;
use crate::message::PubSubMessage;
use redis::Commands;
use std::error::Error;

use tracing::{debug, error, info, trace, warn};

pub fn publish_message(channel: &str, message: ProverMessage) -> Result<(), Box<dyn Error>> {
    info!("#### publishing message to go ####### channel{:?}",channel);
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;
    // println!("publishing message to go channel {:?}", message);
    let serial_data = bincode::serialize(&message).unwrap();
    con.publish(channel, serial_data)?;

    Ok(())
}

pub fn publish_normal_message(message: PubSubMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let json = serde_json::to_string(&message)?;

    con.publish(message.channel, json)?;

    Ok(())
}
