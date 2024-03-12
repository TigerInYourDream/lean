pub use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

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

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefSate::Unshare => {
                self.state.set(RefSate::Share(1));
                Some(Ref { refcell: self })
            }

            RefSate::Share(n) => {
                self.state.set(RefSate::Share(n + 1));
                Some(Ref { refcell: self })
            }

            RefSate::Exclsive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefSate::Unshare = self.state.get() {
            self.state.set(RefSate::Exclsive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

// 如果不借助  Ref RefMut 会出现一个问题  当我们只是增加引用 share(n+1) 或者 exclusive 没办法减少引用
// 特别是进入 exclusive 之后就无法进入其他状态  increase 无法 decrease
// we need to track when it GO AWAY
pub struct Ref<'reference, T> {
    refcell: &'reference RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<'reference, T> Drop for Ref<'reference, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefSate::Unshare | RefSate::Exclsive => unreachable!(),
            RefSate::Share(1) => {
                self.refcell.state.set(RefSate::Unshare);
            }
            RefSate::Share(n) => self.refcell.state.set(RefSate::Share(n - 1)),
        }
    }
}

pub struct RefMut<'reference, T> {
    refcell: &'reference RefCell<T>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // safety
        // a RefMut is only created if on other reference been given out
        // once is give out the state is set Exclusive so NO future reference where give out
        // so we have exclusive lease on inner value, so mutal reference is fine
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<'reference, T> Drop for RefMut<'reference, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefSate::Unshare | RefSate::Share(_) => unreachable!(),
            RefSate::Exclsive => {
                self.refcell.state.set(RefSate::Unshare);
            }
        }
    }
}
