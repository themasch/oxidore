#[macro_use]
pub mod packet;
pub mod utils;
pub mod buffer;
pub mod types;
pub mod field;
pub mod protocol;

use std::net::TcpStream;
use std::io::prelude::*;
use mcnet::utils::read_packet_information;
use mcnet::utils::size_of_varint;
use mcnet::packet::PacketInformation;
use mcnet::protocol::LoginStart;
use mcnet::protocol::ClientHandshake;
use mcnet::protocol::McPacket;
use mcnet::protocol::Packet;

#[derive(Debug)]
pub enum ConnectionState {
    Handshaking,
//    Play,
    Status,
    Login
}


trait PacketFactory {
    fn unpack(info: PacketInformation, data: &[u8]) -> Result<McPacket, ()>;
}

pub struct HandshakingPaketFactory;
impl PacketFactory for HandshakingPaketFactory {
    fn unpack(info: PacketInformation, data: &[u8]) -> Result<McPacket, ()> {
        match info.id {
            0 => { // ClientHandshake
                ClientHandshake::unpack(data)
            },
            _ => {
                panic!("unknown packet_id: {}", info.id);
            }
        }
    }
}
pub struct LoginPaketFactory;
impl PacketFactory for LoginPaketFactory {
    fn unpack(info: PacketInformation, data: &[u8]) -> Result<McPacket, ()> {
        match info.id {
            0 => { // LoginStart
                LoginStart::unpack(data)
            },
            _ => {
                panic!("unknown packet_id: {}", info.id);
            }
        }
    }
}

pub struct Connection<'a> {
    state: ConnectionState,
    stream: &'a mut TcpStream
}




impl<'a> Connection<'a> {
    pub fn create(stream: &mut TcpStream) -> Connection {
        Connection {
            state: ConnectionState::Handshaking,
            stream: stream
        }
    }

    pub fn read_packet(&self, info: PacketInformation, data: &[u8]) -> Result<McPacket, ()> {
        println!("reading packet_id {} in {:?} state", info.id, self.state);
        match self.state {
            ConnectionState::Handshaking => HandshakingPaketFactory::unpack(info, data),
            ConnectionState::Login => LoginPaketFactory::unpack(info, data),
            _ => {
                panic!("not supported yet");
            }
        }
    }

    pub fn handle_packet(&mut self, packet: Result<McPacket, ()>) {
        println!("got {:?}", packet);
        match packet {
            Ok(McPacket::ClientHandshake(handshake)) => {
                match handshake.next_state.get_value() {
                    1 => {
                        self.state = ConnectionState::Status;
                        println!("new state: {:?}", self.state);
                    },
                    2 => {
                        self.state = ConnectionState::Login;
                        println!("new state: {:?}", self.state);
                    }
                    _ => {
                        panic!("unsupported state: {:?}", handshake.next_state);
                    }
                }
            },
            Ok(McPacket::LoginStart(_)) => {

            },
            Err(_) => {
                println!("ERROR DECODING");
            }
        }
    }

    /*pub fn send_packet<P: Packet>(&mut self, packet: &P) {
        let data = packet.pack();

    }*/

    pub fn handle_client(&mut self) {
        let mut buffer = [0; 32];

        loop {
            match self.stream.read(&mut buffer) {
                Ok(0) => break,

                Ok(bytes_read) => {
                    let info = read_packet_information(&buffer);
                    println!("read {} bytes: {:?}", bytes_read, buffer);
                    println!("length: {}, id: {}", info.length, info.id);

                    let data_offset = (size_of_varint(info.length) + 1) as usize;
                    //let data_size   = info.length as usize - size_of_varint(info.id) as usize;
                    let data_end    = size_of_varint(info.length) + info.length as usize;
                    let left_to_read : i64 = info.length as i64 - bytes_read as i64 - size_of_varint(info.length) as i64;
                    {
                        let data = &buffer[(data_offset as usize)..data_end];
                        let packet = self.read_packet(info, data);
                        self.handle_packet(packet);
                    }

                    if left_to_read > 0 {
                        println!("need to read {} more", left_to_read);
                    }
                    buffer = [0; 32];
                },

                Err(err) => {
                    println!("{:?}", err);
                    break;
                }
            }
        }

        println!("ending");
    }
}
