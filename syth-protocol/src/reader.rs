struct Reader {
    data: [u8],
}

impl Reader {
    pub fn read_string(&self) -> String {
        String::from_utf8_lossy(&self.data).trim_end().to_string()
    }
}
