//! Temporary function traits
//!
//! ```
//! use fn_ops::*;
//!
//! struct Predicate(i32);
//!
//! impl FnOnce<(i32, i32)> for Predicate {
//!     type Output = bool;
//!
//!     fn call_once(self, (x, y): (i32, i32)) -> bool {
//!         x * self.0 == y
//!     }
//! }
//!
//! assert!(Predicate(2).call_once((1, 2)))
//! ```

use std::ops;


pub trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

macro_rules! impl_fn {
    () => {
        impl<F, Output> FnOnce<()> for F where
            F: ops::FnOnce() -> Output,
        {
            type Output = Output;
            #[inline]
            fn call_once(self, (): ()) -> Self::Output { self() }
        }

        impl<F, Output> FnMut<(),> for F where
            F: ops::FnMut() -> Output,
        {
            #[inline]
            fn call_mut(&mut self, (): ()) -> Output { self() }
        }

        impl<F, Output> Fn<()> for F where
            F: ops::Fn() -> Output,
        {
            #[inline]
            fn call(&self, (): ()) -> Output { self() }
        }
    };
    ($($arg:ident: $Arg:ident),*) => {
        impl<F, $($Arg,)* Output> FnOnce<($($Arg),*,)> for F where
            F: ops::FnOnce($($Arg),*) -> Output,
        {
            type Output = Output;
            #[inline]
            fn call_once(self, ($($arg),*,): ($($Arg),*,)) -> Self::Output {
                self($($arg),*)
            }
        }

        impl<F, $($Arg,)* Output> FnMut<($($Arg),*,)> for F where
            F: ops::FnMut($($Arg),*) -> Output,
        {
            #[inline]
            fn call_mut(&mut self, ($($arg),*,): ($($Arg),*,)) -> Output {
                self($($arg),*)
            }
        }

        impl<F, $($Arg,)* Output> Fn<($($Arg),*,)> for F where
            F: ops::Fn($($Arg),*) -> Output,
        {
            #[inline]
            fn call(&self, ($($arg),*,): ($($Arg),*,)) -> Output {
                self($($arg),*)
            }
        }
    };
}

impl_fn!();
impl_fn!(a: A);
impl_fn!(a: A, b: B);
impl_fn!(a: A, b: B, c: C);
impl_fn!(a: A, b: B, c: C, d: D);
impl_fn!(a: A, b: B, c: C, d: D, e: E);


#[cfg(test)]
mod tests {
    use super::*;

    fn call_polymorphic<'a, T, U, F>(t: &'a T, u: &'a U, mut f: F) -> bool where
        F: Fn<(&'a T, &'a U), Output = bool>,
    {
        f.call((t, u)) && f.call_mut((t, u)) && f.call_once((t, u))
    }

    #[test]
    fn test_polymorphism() {
        assert!(call_polymorphic(&1, &1, |t, u| t == u));
    }
}
