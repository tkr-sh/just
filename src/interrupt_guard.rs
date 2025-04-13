use super::*;

pub struct InterruptGuard;

impl InterruptGuard {
    pub fn new() -> Self {
        InterruptHandler::instance().block();
        Self
    }
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        InterruptHandler::instance().unblock();
    }
}
