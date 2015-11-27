use mcnet::packet::PacketInformation;

/// reads generic information about a packet
pub fn read_packet_information(data: &[u8]) -> PacketInformation {
    let mut idx = 0;

    while idx < data.len() && data[idx] > 128 {
        idx = idx + 1;
    }

    let length = read_varint(&data[0..idx+1]);
    let packet_id = data[idx+1] as u8; //read_varint(&data[idx+1..idx+8]);

    PacketInformation {
        length: length,
        id: packet_id
    }
}

pub fn size_of_string(str: &String) -> usize {
    let size = str.len();
    return size + size_of_varint(size as u64);
}

pub fn size_of_varint(value: u64) -> usize {
    let mut curr = value;
    let mut length = 0;
    while curr > 0 {
        curr = curr >> 7;
        length = length + 1;
    }

    if length == 0 {
        length = 1
    }

    return length;
}

/// reads a single varint of up to 8 bytes (u64)
pub fn read_varint(data: &[u8]) -> u64 {
    let mut result : u64 = 0;
    //let mut value = 128;
    let mut i = 0;
    loop {
        let value = data[i] as u64;
        result = result + ((value & 127) << (7 * i));
        i = i + 1;
        if value < 128 {
            break;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_varint_300() {
        let input = [0xac, 0x02, 0x00, 0x00, 0x00];
        let number = read_varint(&input);
        assert_eq!(300, number);
    }
}
