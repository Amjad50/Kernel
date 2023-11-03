use core::{cell::Cell, sync::atomic::AtomicI64};

use crate::cpu;

use super::lock;

/// A mutex that can be intered more than once by the same CPU
/// 
/// Only provides `Deref`, and not `DerefMut`, because the data
/// would then be mutated with inconsistent data.
/// Use `Cell` or `RefCell` to allow mutation.
pub struct ReMutex<T> {
    lock: lock::Lock,
    owner_cpu: AtomicI64,
    lock_count: Cell<usize>,
    data: T,
}

unsafe impl<T: Send> Send for ReMutex<T> {}
unsafe impl<T: Send> Sync for ReMutex<T> {}

#[must_use]
pub struct ReMutexGuard<'a, T: 'a> {
    lock: &'a ReMutex<T>,
}

impl<T> ReMutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            lock: lock::Lock::new(),
            owner_cpu: AtomicI64::new(-1),
            lock_count: Cell::new(0),
            data,
        }
    }

    pub fn lock(&mut self) -> ReMutexGuard<T> {
        let cpu_id = cpu::cpu_id() as i64;

        if self.owner_cpu.load(core::sync::atomic::Ordering::Relaxed) == cpu_id {
            self.lock_count.set(
                self.lock_count
                    .get()
                    .checked_add(1)
                    .expect("ReMutex lock count overflow"),
            );
            ReMutexGuard { lock: self }
        } else {
            self.lock.lock();
            self.owner_cpu
                .store(cpu_id, core::sync::atomic::Ordering::Relaxed);
            self.lock_count.set(1);
            ReMutexGuard { lock: self }
        }
    }
}

impl<T> core::ops::Deref for ReMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.lock.data
    }
}

impl<T> Drop for ReMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.lock_count.set(
            self.lock
                .lock_count
                .get()
                .checked_sub(1)
                .expect("ReMutex lock count underflow"),
        );
        if self.lock.lock_count.get() == 0 {
            self.lock
                .owner_cpu
                .store(-1, core::sync::atomic::Ordering::Relaxed);
            self.lock.lock.unlock();
        }
    }
}
