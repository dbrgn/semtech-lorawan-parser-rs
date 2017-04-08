use std::fmt;

use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ProtocolVersion {
    V1,
    V2,
    Other(u8),
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProtocolVersion::V1 => write!(f, "v1"),
            ProtocolVersion::V2 => write!(f, "v2"),
            ProtocolVersion::Other(v) => write!(f, "Other({})", v),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PushData<'a> {
    pub version: ProtocolVersion,
    pub random_token: (u8, u8),
    pub gateway_uid: &'a [u8],
    pub payload: Payload,
}

#[derive(Debug, PartialEq)]
pub struct PushAck {
    pub version: ProtocolVersion,
    pub random_token: (u8, u8),
}

#[derive(Debug, PartialEq)]
pub enum Packet<'a> {
    PushData(PushData<'a>),
    PushAck(PushAck),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Payload {
    pub rxpk: Option<Vec<Rxpk>>,
    pub stat: Option<Stat>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Rxpk {
    pub time: String,
    pub tmst: u32,
    pub freq: f32,
    pub chan: u32,
    pub rfch: u32,
    pub stat: i8,
    pub modu: String,
    pub datr: Value,
    pub codr: String,
    pub rssi: i32,
    pub lsnr: f32,
    pub size: u32,
    pub data: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Stat {
    pub time: String,
    pub lati: f32,
    pub long: f32,
    pub alti: i32,
    pub rxnb: u32,
    pub rxok: u32,
    pub rxfw: u32,
    pub ackr: f32,
    pub dwnb: u32,
    pub txnb: u32,
}
