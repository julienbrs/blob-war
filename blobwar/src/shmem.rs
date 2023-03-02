//! Provide methods sharing a `Movement` in shared memory between processes.
//! Slave anytime process writes new moves atomically and master process only reads them.
use super::configuration::Movement;
use libc::off_t;
use libc::{c_void, size_t};
use nix;
use nix::fcntl::{O_CREAT, O_RDWR};
use nix::sys::mman::MAP_SHARED;
use nix::sys::mman::{mmap, munmap};
use nix::sys::mman::{shm_open, shm_unlink};
use nix::sys::mman::{PROT_READ, PROT_WRITE};
use nix::sys::stat::{S_IRUSR, S_IWUSR};
use nix::unistd::{close, ftruncate};
use std::os::unix::io::RawFd;
use std::ptr;

use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};

struct InnerAtomicMove {
    movements: [Option<Movement>; 2],
    selected: AtomicUsize,
}

pub struct AtomicMove {
    fd: RawFd,
    created: bool,
    address: *mut c_void,
}

impl Drop for AtomicMove {
    fn drop(&mut self) {
        let size = mem::size_of::<InnerAtomicMove>();
        close(self.fd).unwrap();
        munmap(self.address, size as size_t).unwrap();

        if self.created {
            shm_unlink("blobwar").unwrap();
        }
    }
}

impl AtomicMove {
    pub fn new() -> Result<Self, nix::Error> {
        let size = mem::size_of::<InnerAtomicMove>();
        let fd = shm_open("blobwar", O_CREAT | O_RDWR, S_IRUSR | S_IWUSR)?;
        ftruncate(fd, size as off_t)?;

        let address = mmap(
            ptr::null_mut(),
            size as size_t,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )?;

        let atomic: &mut InnerAtomicMove =
            unsafe { (address as *mut InnerAtomicMove).as_mut().unwrap() };
        atomic.movements[0] = None;
        atomic.movements[1] = None;
        atomic.selected = Default::default();

        Ok(AtomicMove {
            fd,
            address,
            created: true,
        })
    }

    pub fn connect() -> Result<Self, nix::Error> {
        let size = mem::size_of::<InnerAtomicMove>();
        let fd = shm_open("blobwar", O_RDWR, S_IRUSR | S_IWUSR)?;
        let address = mmap(
            ptr::null_mut(),
            size as size_t,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )?;
        Ok(AtomicMove {
            fd,
            address,
            created: false,
        })
    }

    pub fn store(&mut self, movement: Option<Movement>) {
        let atomic: &mut InnerAtomicMove =
            unsafe { (self.address as *mut InnerAtomicMove).as_mut().unwrap() };
        let index = atomic.selected.load(Ordering::SeqCst);
        atomic.movements[(index + 1) % 2] = movement;
        atomic.selected.fetch_add(1, Ordering::SeqCst);
    }

    pub fn load(&self) -> Option<Movement> {
        let atomic: &InnerAtomicMove =
            unsafe { (self.address as *const InnerAtomicMove).as_ref().unwrap() };
        atomic.movements[atomic.selected.load(Ordering::SeqCst) % 2]
    }
}
