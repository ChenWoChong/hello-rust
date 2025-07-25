use std::fs::File;
use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
#[allow(dead_code)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn new(_addr: &str) -> MyWriter<BufWriter<TcpStream>> {
        let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        MyWriter {
            writer: BufWriter::new(stream),
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

#[allow(dead_code)]
fn test_my_writer() {
    let mut writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
    _ = writer.write("hello world!");
}

#[allow(dead_code)]
pub fn test_dyn_writer() {
    let mut f = File::create("./test_write_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello").unwrap();
    // let w1 = w.by_ref();
    let w1 = &mut *w;
    w1.write_all(b" world").unwrap();
}
