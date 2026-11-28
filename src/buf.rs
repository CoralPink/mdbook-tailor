use core::fmt::NumBuffer;

pub struct WriteBuf {
    buf: String,
    num_buf: NumBuffer<usize>,
}

impl WriteBuf {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            num_buf: NumBuffer::new(),
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.buf.push(c);
    }

    pub fn push_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn push_usize(&mut self, v: usize) {
        self.buf.push_str(v.format_into(&mut self.num_buf));
    }

    pub fn into_string(self) -> String {
        self.buf
    }
}
