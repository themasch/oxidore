#[derive(Debug)]
pub struct InputBuffer<'a> {
    data:     &'a [u8],
    position: usize,
    length:   usize
}

impl<'a> InputBuffer<'a> {
    pub fn create(data: &'a [u8]) -> InputBuffer<'a> {
        InputBuffer {
            data:     data,
            position: 0,
            length:   data.len()
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let tmp = self.data[self.position];
        self.position = self.position + 1;
        return tmp;
    }

    pub fn has_next(&self) -> bool {
        self.position < self.length
    }

    pub fn get_length(&self) -> usize {
        self.length
    }
}


pub struct PacketParser<'a> {
    buffer: InputBuffer<'a>
}

impl<'a> PacketParser<'a> {
    pub fn from_bytes(data: &'a [u8]) -> PacketParser<'a> {
        let buffer = InputBuffer::create(data);
        PacketParser {
            buffer: buffer
        }
    }

    pub fn varint(&mut self) -> u64 {
        let mut value : u64 = 0;
        let mut idx = 0;
        loop {
            let current = self.buffer.read_byte() as u64;

            let shift_by = if idx > 0 {
                (7 as u64).pow(idx as u32)
            } else {
                0
            };

            value = value + ((current & 127) << shift_by);

            if current < 128 {
                break;
            }

            idx = idx + 1;
        }

        return value
    }

    pub fn string(&mut self) -> String {
        let length = self.varint() as usize;
        let mut buf = Vec::with_capacity(length);
        while buf.len() < length {
            let current = self.buffer.read_byte();
            buf.push(current);
        }

        String::from_utf8(buf).unwrap()
    }

    pub fn u16(&mut self) -> u16 {
        self.buffer.read_byte() as u16 * 256 + self.buffer.read_byte() as u16
    }
}


#[test]
fn test_input_buffer_create() {
    let inpt: [u8; 9] =  [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let buff = InputBuffer::create(&inpt);
    assert_eq!(buff.get_length(), 9);
}

#[test]
fn test_input_buffer_read_all() {
    let inpt: [u8; 9] =  [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut buff = InputBuffer::create(&inpt);
    for x in 1..10 {
        assert!(buff.has_next());
        assert_eq!(buff.read_byte(), x);
    }
    assert!(!buff.has_next());
}

