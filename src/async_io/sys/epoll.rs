use std::io;
use std::os::unix::io::RawFd;

pub struct Reactor {}

impl Reactor {
    pub fn new() -> io::Result<Reactor> {
        unimplemented!();
    }

    pub fn interest(&self, fd: RawFd, key: usize, read: bool, write: bool) -> io::Result<()> {
        unimplemented!();
    }
}