pub struct OutputBuffer<'a> {
    buffer: &'a mut [u8],
    position: usize,
    length:   usize
}

impl<'a> OutputBuffer<'a> {
    pub fn from_slice(slice: &'a mut [u8]) -> OutputBuffer<'a> {
        let length = slice.len();
        OutputBuffer {
            buffer: slice,
            position: 0,
            length: length
        }
    }

    pub fn from_vector(vec: &'a mut Vec<u8>) -> OutputBuffer<'a> {
        let length = vec.len();

        OutputBuffer {
            buffer: &mut vec[..],
            position: 0,
            length: length
        }
    }

    pub fn put_byte(&mut self, data: u8) {
        if self.position == self.length {
            panic!("trying to put a byte in a full buffer");
        }

        self.buffer[self.position] = data;
        self.position = self.position + 1;
    }
}

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

