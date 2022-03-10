extern crate redis;
use bincode;
use crate::message::ProverMessage;
use crate::message::PubSubMessage;
use redis::Commands;
use std::error::Error;

pub fn publish_message(message: ProverMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let serial_data = binary::serialize(&message)?;

    con.publish(message.channel, serial_data)?;

    Ok(())
}

pub fn publish_normal_message(message: PubSubMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let json = serde_json::to_string(&message)?;

    con.publish(message.channel, json)?;

    Ok(())
}