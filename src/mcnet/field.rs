use mcnet::types::VarInt;
use mcnet::input::OutputBuffer;
use mcnet::input::InputBuffer;

pub trait PacketField<'a> {
    fn write_to(&self, buffer: &mut OutputBuffer);
    fn read_from(buffer: &mut InputBuffer) -> Self;
}

impl<'a> PacketField<'a> for u8 {
    fn write_to(&self, mut buffer: &mut OutputBuffer) {
        buffer.put_byte(*self);
    }

    fn read_from(buffer: &mut InputBuffer) -> u8 {
        buffer.read_byte()
    }
}

impl<'a> PacketField<'a> for u16 {
    fn write_to(&self, mut buffer: &mut OutputBuffer) {
        buffer.put_byte((*self >> 8) as u8);
        buffer.put_byte((*self & 0xff) as u8);
    }

    fn read_from(buffer: &mut InputBuffer) -> u16 {
        buffer.read_byte() as u16 * 256 + buffer.read_byte() as u16
    }
}

impl<'a> PacketField<'a> for VarInt {
    fn write_to(&self, mut buffer: &mut OutputBuffer) {
        let mut val = self.get_value();
        while val > 127 {
            let x = (val & 127) as u8 | 128;
            buffer.put_byte(x);
            val = val >> 7;
        }

        buffer.put_byte(val as u8);
    }

    fn read_from(buffer: &mut InputBuffer) -> VarInt {
        let mut value : u32 = 0;
        let mut idx = 0;
        loop {
            let current = buffer.read_byte() as u32;

            let shift_by = if idx > 0 {
                (7 as u32).pow(idx as u32)
            } else {
                0
            };

            value = value + ((current & 127) << shift_by);

            if current < 128 {
                break;
            }

            idx = idx + 1;
        }

        VarInt::wrap(value)
    }
}

impl<'a> PacketField<'a> for String {
    fn write_to(&self, mut buffer: &mut OutputBuffer) {
        VarInt::wrap(self.len() as u32).write_to(&mut buffer);
        let bytes = self.as_bytes();
        for i in 0..bytes.len() {
            buffer.put_byte(bytes[i]);
        }
    }

    fn read_from(buffer: &mut InputBuffer) -> String {
        let length_var : VarInt = PacketField::read_from(buffer);
        let length = length_var.get_value();
        let mut buf = Vec::with_capacity(length as usize);
        while buf.len() < length as usize {
            let current = buffer.read_byte();
            buf.push(current);
        }

        String::from_utf8(buf).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use mcnet::types::VarInt;
    use mcnet::input::OutputBuffer;

    #[test]
    fn test_write_varint_1() {
        let mut buffer_slice = [0u8; 5];
        {
            let mut buffer = OutputBuffer::from_slice(&mut buffer_slice);
            VarInt::wrap(1).write_to(&mut buffer);
        }
        assert_eq!([0x01, 0x00, 0x00, 0x00, 0x00], buffer_slice);
    }

    #[test]
    fn test_write_varint_10() {
        let mut buffer_slice = [0u8; 5];
        {
            let mut buffer = OutputBuffer::from_slice(&mut buffer_slice);
            VarInt::wrap(10).write_to(&mut buffer);
        }
        assert_eq!([0x0A, 0x00, 0x00, 0x00, 0x00], buffer_slice);
    }

    #[test]
    fn test_write_varint_100() {
        let mut buffer_slice = [0u8; 5];
        {
            let mut buffer = OutputBuffer::from_slice(&mut buffer_slice);
            VarInt::wrap(100).write_to(&mut buffer);
        }
        assert_eq!([100, 0x00, 0x00, 0x00, 0x00], buffer_slice);
    }

    #[test]
    fn test_write_varint_300() {
        let mut buffer_slice = [0u8; 5];
        {
            let mut buffer = OutputBuffer::from_slice(&mut buffer_slice);
            VarInt::wrap(300).write_to(&mut buffer);
        }
        assert_eq!([0xac, 0x02, 0x00, 0x00, 0x00], buffer_slice);
    }
}
