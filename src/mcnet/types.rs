#[derive(PartialEq,Debug)]
pub struct VarInt(u32);

impl VarInt {
    pub fn wrap(value: u32) -> VarInt {
        VarInt(value)
    }

    pub fn get_value(&self) -> u32 {
        self.0
    }

    pub fn byte_len(&self) -> usize {
        let mut curr = self.get_value();
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
}
