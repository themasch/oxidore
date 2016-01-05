use mcnet::utils::size_of_varint;
use mcnet::buffer::InputBuffer;
use mcnet::buffer::OutputBuffer;
use mcnet::types::VarInt;
use mcnet::field::PacketField;
use mcnet::packet::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ConnectionState {
    Handshaking,
    Play,
    Status,
    Login
}

pub trait Packet {
    fn unpack(data: &[u8]) -> Result<McPacket, ()>;

    fn pack(& self) -> Vec<u8>;
}

pub struct Protocol {
    pub package_map: HashMap<ConnectionState, HashMap<u8, Box<Fn(&[u8]) -> McPacket>>>
}

protocol!{
    state Handshaking {
        package ClientHandshake id 0x00
        {
            protocol_version: VarInt,
            address: String,
            port: u16,
            next_state:  VarInt
        }
    }

    state Login {
        package LoginStart id 0x00
        {
            name: String
        }
    }
}
