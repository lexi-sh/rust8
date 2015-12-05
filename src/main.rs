
mod chip8;
use chip8::Cpu;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut cpu = Cpu::new();
    let path = Path::new("/tmp/brix");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    cpu.run(file);
}
