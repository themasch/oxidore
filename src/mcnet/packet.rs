use mcnet::utils::size_of_string;
use mcnet::utils::size_of_varint;
use mcnet::input::InputBuffer;
use mcnet::input::OutputBuffer;
use mcnet::types::VarInt;
use mcnet::field::PacketField;

pub struct PacketInformation {
    pub length: u64,
    pub id: u8
}


pub trait Packet {
    fn unpack(data: &[u8]) -> ServerPacket;

    fn pack(& self) -> Vec<u8>;
}

#[derive(Debug,PartialEq)]
pub enum ServerPacket {
    ClientHandshake(ClientHandshake),
    LoginStart(LoginStart)
}

#[derive(Debug,PartialEq)]
pub struct ClientHandshake {
    pub protocol_version: VarInt,
    pub address: String,
    pub port: u16,
    pub next_state:  VarInt
}

impl Packet for ClientHandshake {
    fn unpack(data: &[u8]) -> ServerPacket {
        let mut buffer = InputBuffer::create(data);
        ServerPacket::ClientHandshake(ClientHandshake {
            protocol_version: PacketField::read_from(&mut buffer),
            address:    PacketField::read_from(&mut buffer),
            port:       PacketField::read_from(&mut buffer),
            next_state: PacketField::read_from(&mut buffer)
        })
    }

    fn pack(&self) -> Vec<u8> {
        let length : usize = size_of_string(&self.address)
                        + self.protocol_version.byte_len()
                        + 2 // 16bits for port
                        + self.next_state.byte_len()
                        + 1; // packet id

        let size = length + size_of_varint(length as u64);
        let mut buffer_vec : Vec<u8> = vec![0; size];
        {
            let mut buffer = OutputBuffer::from_vector(&mut buffer_vec);
            VarInt::wrap(length as u32).write_to(&mut buffer);
            (0x00 as u8).write_to(&mut buffer);
            self.protocol_version.write_to(&mut buffer);
            self.address.write_to(&mut buffer);
            self.port.write_to(&mut buffer);
            self.next_state.write_to(&mut buffer);
        }

        return buffer_vec;
    }
}

#[derive(Debug,PartialEq)]
pub struct LoginStart {
    pub name: String
}

impl LoginStart {
    pub fn unpack(data: &[u8]) -> ServerPacket {
        let mut buffer = InputBuffer::create(data);
        ServerPacket::LoginStart(LoginStart {
            name: PacketField::read_from(&mut buffer),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use mcnet::types::VarInt;

    #[test]
    fn pack_client_handshake() {
        let result = [15, 0, 27, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 48, 57, 2];
        let hs = ClientHandshake {
            protocol_version: VarInt::wrap(27),
            address: "127.0.0.1".to_string(),
            port: 12345,
            next_state: VarInt::wrap(2)
        };

        let buffer = hs.pack();
        for i in 0..16 {
            assert_eq!(result[i], buffer[i]);
        }


        let res = ClientHandshake::unpack(&buffer[2..]);
        assert_eq!(res, ServerPacket::ClientHandshake(hs));
    }

     #[test]
    fn pack_client_handshake_with_hostname() {
        let result = [ 26, 0, 27, 20, 109, 105, 110, 101, 99, 114, 97, 102, 116,
                       46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 0, 80, 2];
        let hs = ClientHandshake {
            protocol_version: VarInt::wrap(27),
            address: "minecraft.google.com".to_string(),
            port: 80,
            next_state: VarInt::wrap(2)
        };

        let buffer = hs.pack();
        for i in 0..26 {
            assert_eq!(result[i], buffer[i]);
        }

        let res = ClientHandshake::unpack(&buffer[2..]);
        assert_eq!(res, ServerPacket::ClientHandshake(hs));
    }
}
