pub use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[derive(Clone, Copy)]
pub enum RefSate {
    Unshare,
    Share(isize),
    Exclsive,
}

/// implied by UnsafeCell
/// Impl<T> !Sync for RefCell<T> {}
/// not sync because Share(N) cant not work in multi thread
struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefSate>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefSate::Unshare),
        }
    }

    pub fn borrow(&self) -> Option<&T> {
        match self.state.get() {
            RefSate::Unshare => {
                self.state.set(RefSate::Share(1));
                Some(unsafe { &*self.value.get() })
            }

            RefSate::Share(n) => {
                self.state.set(RefSate::Share(n + 1));
                Some(unsafe { &*self.value.get() })
            }

            RefSate::Exclsive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        if let RefSate::Unshare = self.state.get() {
            self.state.set(RefSate::Exclsive);
            Some(unsafe { &mut *self.value.get() })
        } else {
            None
        }
    }
}
