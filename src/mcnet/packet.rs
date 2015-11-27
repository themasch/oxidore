use mcnet::utils::size_of_string;
use mcnet::utils::size_of_varint;
use mcnet::utils::write_varint;
use mcnet::utils::write_string;
use mcnet::input::PacketParser;

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
    pub protocol_version: u64,
    pub address: String,
    pub port: u16,
    pub next_state:  u64
}

impl Packet for ClientHandshake {
    fn unpack(data: &[u8]) -> ServerPacket {
        let mut buffer = PacketParser::from_bytes(data);
        ServerPacket::ClientHandshake(ClientHandshake {
            protocol_version: buffer.varint(),
            address: buffer.string(),
            port: buffer.u16(),
            next_state: buffer.varint()
        })
    }

    fn pack(&self) -> Vec<u8> {
        let length : usize = size_of_string(&self.address)
                        + size_of_varint(self.protocol_version)
                        + 2 // 16bits for port
                        + size_of_varint(self.next_state)
                        + 1; // packet id

        let size = length + size_of_varint(length as u64);
        let mut buffer : Vec<u8> = vec![0; size];
        {
            let mut buffer_slice = &mut buffer[..];

            let mut idx = write_varint(length as u64, &mut buffer_slice);
            buffer_slice[idx] = 0x00; // packet id
            idx = idx + 1;
            idx = idx + write_varint(self.protocol_version, &mut buffer_slice[idx..]);
            idx = idx + write_string(&self.address, &mut buffer_slice[idx..]);
            buffer_slice[idx] = (self.port >> 8) as u8;
            buffer_slice[idx+1] = (self.port & 0xff) as u8;
            idx = idx + 2;
            write_varint(self.next_state, &mut buffer_slice[idx..]);

        }
        return buffer;
    }
}

#[derive(Debug,PartialEq)]
pub struct LoginStart {
    pub name: String
}

impl LoginStart {
    pub fn unpack(data: &[u8]) -> ServerPacket {
        let mut buffer = PacketParser::from_bytes(data);
        ServerPacket::LoginStart(LoginStart {
            name: buffer.string()
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_client_handshake() {
        let result = [15, 0, 27, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 48, 57, 2];
        let hs = ClientHandshake {
            protocol_version: 27,
            address: "127.0.0.1".to_string(),
            port: 12345,
            next_state: 2
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
            protocol_version: 27,
            address: "minecraft.google.com".to_string(),
            port: 80,
            next_state: 2
        };

        let buffer = hs.pack();
        for i in 0..26 {
            assert_eq!(result[i], buffer[i]);
        }

        let res = ClientHandshake::unpack(&buffer[2..]);
        assert_eq!(res, ServerPacket::ClientHandshake(hs));
    }
}
