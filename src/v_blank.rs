use std::sync::{
    Arc, Condvar, Mutex,
    atomic::{AtomicBool, Ordering::Relaxed},
};

pub struct VBlank {
    flag: Mutex<bool>,
    condvar: Condvar,
}

impl VBlank {
    pub const fn new() -> Self {
        Self {
            flag: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    pub fn send(&self) {
        *self.flag.lock().unwrap_or_else(|e| e.into_inner()) = true;
        self.condvar.notify_all();
    }

    pub fn wait(&self) {
        let mut flag = self.flag.lock().unwrap_or_else(|e| e.into_inner());
        while !*flag {
            flag = self.condvar.wait(flag).unwrap_or_else(|e| e.into_inner());
        }
        *flag = false;
    }
}
