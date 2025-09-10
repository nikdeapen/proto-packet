use std::io::Write;

/// The `Write` implementation for services.
#[derive(Default)]
pub struct ServiceWrite {
    buffer: Vec<u8>,
}

impl ServiceWrite {
    //! Body

    pub fn body(self) -> Vec<u8> {
        self.buffer
    }
}

impl Write for ServiceWrite {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
