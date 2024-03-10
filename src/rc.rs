use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refconter: Cell<isize>,
}

// 不能直接在 Rc中保存计数的原因是 当clone Rc 相当于每个副本都有一个 conter
// 那我们如何之后一个 Rc对象被 clone被引用了多少次呢
struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refconter: Cell::new(1),
        });

        // 下面这么写 当我们结束这个方法 Box被回收 inner指针也没有了
        // 即便是 Box被回收 我们仍然需要这个指针
        // Rc {
        //     inner: &*inner
        // }

        // Box::into_raw(inner) 将 Box 中的数据移动到堆内存，并返回一个原始指针。这样，即使 Box 被回收，数据仍然存在于堆内存中，可以通过原始指针访问
        Rc {
            // SAFETY: Box doesnot give as a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: inner is Box that only deallocated when the last Rc is gone
        // we have an Rc, therefore that Box has not be deallocated It's fine to Derf
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refconter.get();
        inner.refconter.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refconter.get();
        if c == 1 {
            // SAFETY: we are the only Rc left, so we are being droped
            // there for after us, there will be no Rc's and no reference to T
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other Rc's left, so we just decrement the refcount
            inner.refconter.set(c - 1);
        }
    }
}
