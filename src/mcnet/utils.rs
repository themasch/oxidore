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

pub fn write_string(value: &String, mut buffer: &mut [u8]) -> usize {
    let string_start = write_varint(value.len() as u64, &mut buffer);
    let bytes = value.as_bytes();
    for i in 0..bytes.len() {
        buffer[i + string_start] = bytes[i];
    }
    return bytes.len() + string_start;
}

pub fn write_varint(value: u64, buffer: &mut [u8]) -> usize {
    let mut val = value;
    let mut idx : usize = 0;
    while val > 127 {
        let x = (val & 127) as u8 | 128;
        buffer[idx] = x;
        idx = idx + 1;
        val = val >> 7;
    }

    buffer[idx] = val as u8;

    return idx + 1;
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
    fn test_write_varint_1() {
        let mut buffer = [0u8; 5];
        let written = write_varint(1, &mut buffer);
        assert_eq!(1, written);
        assert_eq!([0x01, 0x00, 0x00, 0x00, 0x00], buffer);
    }

    #[test]
    fn test_write_varint_10() {
        let mut buffer = [0u8; 5];
        let written = write_varint(10, &mut buffer);
        assert_eq!(1, written);
        assert_eq!([0x0a, 0x00, 0x00, 0x00, 0x00], buffer);
    }


    #[test]
    fn test_write_varint_100() {
        let mut buffer = [0u8; 5];
        let written = write_varint(100, &mut buffer);
        assert_eq!(1, written);
        assert_eq!([100, 0x00, 0x00, 0x00, 0x00], buffer);
    }


    #[test]
    fn test_write_varint_300() {
        let mut buffer = [0u8; 5];
        let written = write_varint(300, &mut buffer);
        assert_eq!(2, written);
        assert_eq!([0xac, 0x02, 0x00, 0x00, 0x00], buffer);
    }

    #[test]
    fn test_read_varint_300() {
        let input = [0xac, 0x02, 0x00, 0x00, 0x00];
        let number = read_varint(&input);
        assert_eq!(300, number);
    }

    #[test]
    fn test_write_and_read_varint() {
        // buffer kann wiederverwendet werden da die zahl immer länger(größer) wird.
        let mut buffer = [0u8; 4];
        for i in 0..67_108_864 {
            write_varint(i, &mut buffer);
            let result = read_varint(&buffer);
            assert_eq!(i, result);
        }
    }
}
