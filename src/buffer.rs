#[derive(Clone)]
pub struct String<const CAPACITY: usize> {
    data: [u16; CAPACITY],
    len: usize,
}

impl<const CAPACITY: usize> String<CAPACITY> {
    pub fn new() -> Self {
        String {
            data: [0u16; CAPACITY],
            len: 0,
        }
    }

    pub fn append_str(self, s: &str) -> Self {
        self.append_iter_u16(s.encode_utf16())
    }

    pub fn append_iter_u16<'a, T: Iterator<Item = u16>>(mut self, mut iter: T) -> Self {
        loop {
            match iter.next() {
                Some(c) => self = self.append_u16(c),
                None => return self,
            }
        }
    }

    pub fn append_ptr_u16(mut self, mut s: *const u16) -> Self {
        loop {
            let c = unsafe { *s };
            self = self.append_u16(c);
            match c {
                0 => return self,
                _ => s = unsafe { s.add(1) },
            }
        }
    }

    // handles buffer size and terminating zero
    fn append_u16(mut self, c: u16) -> Self {
        if self.len >= CAPACITY {
            panic!("Buffer overflow");
        }

        // last char in buffer is always zero
        let d = if self.len >= CAPACITY - 1 { 0 } else { c };

        self.data[self.len] = d;

        // don't count terminator
        if d != 0 {
            self.len += 1;
        };

        self
    }

    pub fn as_iter<'a>(&'a self) -> impl Iterator<Item = u16> + 'a {
        self.data[..self.len].iter().copied()
    }

    pub fn as_ptr(self) -> *const u16 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(mut self) -> *mut u16 {
        self.data.as_mut_ptr()
    }
}
