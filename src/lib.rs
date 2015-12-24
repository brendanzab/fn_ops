// Copyright 2015 Brendan Zabarauskas
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Temporary traits for function operator overloading, pending the stabilization of `Fn`, `FnMut`,
//! and `FnOnce`.
//!
//! ```rust
//! use fn_ops::*;
//!
//! struct IsMultipleOf(i32);
//!
//! impl FnOnce<(i32, i32)> for IsMultipleOf {
//!     type Output = bool;
//!
//!     fn call_once(self, (x, y): (i32, i32)) -> bool { self.call((x, y)) }
//! }
//!
//! impl FnMut<(i32, i32)> for IsMultipleOf {
//!     fn call_mut(&mut self, (x, y): (i32, i32)) -> bool { self.call((x, y)) }
//! }
//!
//! impl Fn<(i32, i32)> for IsMultipleOf {
//!     fn call(&self, (x, y): (i32, i32)) -> bool {
//!         x * self.0 == y
//!     }
//! }
//!
//! fn assert_fn<Args, F: Fn<Args, Output = bool>>(args: Args, f: F) {
//!     assert!(f.call(args))
//! }
//!
//! assert_fn((1, 2), IsMultipleOf(2));
//! assert_fn((1, 2, 3), |x, y, z| x != y && y != z && z != x);
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
            fn call_once(self, ($($arg),*,): ($($Arg),*,)) -> Self::Output { self($($arg),*) }
        }

        impl<F, $($Arg,)* Output> FnMut<($($Arg),*,)> for F where
            F: ops::FnMut($($Arg),*) -> Output,
        {
            #[inline]
            fn call_mut(&mut self, ($($arg),*,): ($($Arg),*,)) -> Output { self($($arg),*) }
        }

        impl<F, $($Arg,)* Output> Fn<($($Arg),*,)> for F where
            F: ops::Fn($($Arg),*) -> Output,
        {
            #[inline]
            fn call(&self, ($($arg),*,): ($($Arg),*,)) -> Output { self($($arg),*) }
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
