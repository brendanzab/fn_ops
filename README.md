# fn_ops

[![Build Status](https://travis-ci.org/bjz/fn_ops.svg?branch=master)](https://travis-ci.org/bjz/fn_ops)
[![Version](https://img.shields.io/crates/v/fn_ops.svg)](https://crates.io/crates/fn_ops)
[![License](https://img.shields.io/crates/l/fn_ops.svg)](https://github.com/bjz/fn_ops/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/fn_ops.svg)](https://crates.io/crates/fn_ops)

[Documentation](http://bjz.github.io/fn_ops)

Temporary traits for function operator overloading, pending the stabilization of
`Fn`, `FnMut`, and `FnOnce`.

```rust
use fn_ops::*;

struct IsMultipleOf(i32);

impl FnOnce<(i32, i32)> for IsMultipleOf {
    type Output = bool;

    fn call_once(self, (x, y): (i32, i32)) -> bool { self.call((x, y)) }
}

impl FnMut<(i32, i32)> for IsMultipleOf {
    fn call_mut(&mut self, (x, y): (i32, i32)) -> bool { self.call((x, y)) }
}

impl Fn<(i32, i32)> for IsMultipleOf {
    fn call(&self, (x, y): (i32, i32)) -> bool {
        x * self.0 == y
    }
}

fn assert_fn<Args, F: Fn<Args, Output = bool>>(args: Args, f: F) {
    assert!(f.call(args))
}

assert_fn((1, 2), IsMultipleOf(2));
assert_fn((1, 2, 3), |x, y, z| x != y && y != z && z != x);
```
