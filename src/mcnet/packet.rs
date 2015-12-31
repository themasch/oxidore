use mcnet::utils::size_of_varint;
use mcnet::types::VarInt;

pub struct PacketInformation {
    pub length: u64,
    pub id: u8
}


pub trait Sized {
    fn get_size(&self) -> usize;
}

impl Sized for String {
    fn get_size(&self) -> usize  {
        let size = self.len();
        return size + size_of_varint(size as u64);
    }
}

impl Sized for VarInt {
    fn get_size(&self) -> usize {
        self.byte_len()
    }
}

impl Sized for u8 {
    fn get_size(&self) -> usize { 1 }
}

impl Sized for u16 {
    fn get_size(&self) -> usize { 2 }
}

#[macro_export]
macro_rules! protocol {
    (
        $(
            package $name:ident id $package_id:expr {
                $($field:ident: $ftype:ty),+
            }
        )*
    ) => {
        $(
            #[derive(Debug,PartialEq)]
            pub struct $name {
                $(pub $field: $ftype),+
            }

            impl Packet for $name {
                fn unpack(data: &[u8]) -> Result<McPacket, ()> {
                    let mut buffer = InputBuffer::create(data);
                    Ok(McPacket::$name($name {
                        $($field: PacketField::read_from(&mut buffer)),+
                    }))
                }

                fn pack(&self) -> Vec<u8> {
                    let mut length = 0;
                    $(length = length + self.$field.get_size());+;
                    let size = length + size_of_varint(length as u64);
                    let mut buffer_vec : Vec<u8> = vec![0; size];
                    {
                        let mut buffer = OutputBuffer::from_vector(&mut buffer_vec);
                        VarInt::wrap(length as u32).write_to(&mut buffer);
                        ($package_id as u8).write_to(&mut buffer);

                        $(self.$field.write_to(&mut buffer));+
                    }

                    return buffer_vec;
                }
            }
        )*


        pub trait Packet {
            fn unpack(data: &[u8]) -> Result<McPacket, ()>;

            fn pack(& self) -> Vec<u8>;
        }

        #[derive(Debug,PartialEq)]
        pub enum McPacket {
            $($name($name)),+
        }
    }
}
