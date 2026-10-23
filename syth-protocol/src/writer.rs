struct Writer {
    data: Vec<u8>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    // pub fn write_varint() {

    // }

    pub fn write_string(&mut self, string: &str) {
        let mut b = [b' '; 64];
        let n = string.len().min(64);
        b[..n].copy_from_slice(&string.as_bytes()[..n]);
        self.data.extend_from_slice(&b);
    }
}
