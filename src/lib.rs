//! Provides an unsafe tagless alternative to `Option<T>` that uses less memory.
//!
//! Nightly-only. `#![no_std]`.

#![feature(untagged_unions, const_fn)]

#![no_std]

use core::mem::replace;

/// A union which either holds a `T` or nothing.
///
/// This can be seen as a `T` that may not be properly initialized.
///
/// In contrast to `Option<T>`, this type does not know if it stores a `T` or not, so it relies on
/// the caller to uphold safety. Consequently, `UntaggedOption` can be a bit smaller than `Option`
/// since it never has a discriminant. This makes it useful in resource-constrained environments or
/// other scenarios where the space overhead of `Option` is significant.
///
/// # Examples
///
/// ```
/// # extern crate untagged_option;
/// # use untagged_option::UntaggedOption;
/// # fn main() {
/// let mut opt = UntaggedOption::none();
///
/// // `opt` didn't hold a value before, so this assigment is fine.
/// // If it did, the value would be leaked.
/// opt = UntaggedOption::some("&str stored");
///
/// unsafe {
///     // Safe: `opt` is now properly initialized and holds a value.
///     assert_eq!(opt.as_ref(), &"&str stored");
///     let content = opt.take();   // `opt` is now uninitialized/none
/// }
/// # }
/// ```
///
/// # Safety
///
/// Since `UntaggedOption` does not have a discriminant, the user must know when the option contains
/// a valid value and only call the appropriate methods.
///
/// `UntaggedOption` does not destroy the contained value when dropped (it doesn't know if there
/// *is* a value), so the user must make sure to manually remove the value by calling `take` (or
/// only use `UntaggedOption` with `Copy` types that do not need to be dropped).
///
/// This also applies to assignments: An assignment like `opt = UntaggedOption::none()` will leak
/// the previously contained value (if any).
#[allow(unions_with_drop_fields)]
pub union UntaggedOption<T> {
    pub some: T,
    pub none: (),
}

impl<T> UntaggedOption<T> {
    /// Creates a new `UntaggedOption` holding no value.
    ///
    /// It is not safe to call any method on the resulting `UntaggedOption`.
    pub const fn none() -> Self {
        UntaggedOption {
            none: (),
        }
    }

    /// Creates an `UntaggedOption` containing `t`.
    ///
    /// # Note
    ///
    /// When the `UntaggedOption` is dropped, `t` will *not* be dropped automatically. You must call
    /// `take` if you need `t` to be dropped properly.
    pub const fn some(t: T) -> Self {
        UntaggedOption {
            some: t,
        }
    }

    /// Takes the `T` out of an initialized wrapper, making it uninitialized.
    ///
    /// This can be called to drop the contained `T`.
    ///
    /// # Safety
    ///
    /// Calling this method requires that `self` holds a valid `T`. [`UntaggedOption::some`] creates
    /// such an option.
    ///
    /// [`UntaggedOption::some`]: #method.some
    pub unsafe fn take(&mut self) -> T {
        replace(self, UntaggedOption::none()).some
    }

    /// Obtains an immutable reference to the contained `T`.
    ///
    /// # Safety
    ///
    /// Calling this method requires that `self` holds a valid `T`. [`UntaggedOption::some`] creates
    /// such an option.
    ///
    /// [`UntaggedOption::some`]: #method.some
    pub unsafe fn as_ref(&self) -> &T {
        &self.some
    }

    /// Obtains a mutable reference to the contained `T`.
    ///
    /// # Safety
    ///
    /// Calling this method requires that `self` holds a valid `T`. [`UntaggedOption::some`] creates
    /// such an option.
    ///
    /// [`UntaggedOption::some`]: #method.some
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut self.some
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_context() {
        static mut MY_OPT: UntaggedOption<u8> = UntaggedOption::none();
        unsafe {
            MY_OPT = UntaggedOption::some(123);
            assert_eq!(*MY_OPT.as_ref(), 123);
            *MY_OPT.as_mut() = 42;
            assert_eq!(*MY_OPT.as_ref(), 42);
            MY_OPT.take();
        }
    }

    #[test]
    fn correct_drop() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static DROPCOUNT: AtomicUsize = AtomicUsize::new(0);

        struct MyDrop;
        impl Drop for MyDrop {
            fn drop(&mut self) {
                DROPCOUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        let mut opt = UntaggedOption::some(MyDrop);
        drop(opt);
        assert_eq!(DROPCOUNT.load(Ordering::SeqCst), 0);
        opt = UntaggedOption::some(MyDrop);
        assert_eq!(DROPCOUNT.load(Ordering::SeqCst), 0);
        unsafe { opt.take(); }
        assert_eq!(DROPCOUNT.load(Ordering::SeqCst), 1);
    }
}
