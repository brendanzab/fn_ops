# fn_ops

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
