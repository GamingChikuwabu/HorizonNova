use std::io;

struct Writer;

impl io::Write : Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    
}