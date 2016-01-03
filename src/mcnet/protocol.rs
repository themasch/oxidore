use mcnet::utils::size_of_varint;
use mcnet::buffer::InputBuffer;
use mcnet::buffer::OutputBuffer;
use mcnet::types::VarInt;
use mcnet::field::PacketField;
use mcnet::packet::*;

protocol!{
    package ClientHandshake id 0x00 {
        protocol_version: VarInt,
        address: String,
        port: u16,
        next_state:  VarInt
    }

    package LoginStart id 0x00 {
        name: String
    }
}
