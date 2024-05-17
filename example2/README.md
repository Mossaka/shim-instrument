# Example of shim-instrument

This example demonstrates how to use the `shim-instrument` crate to conditionally compile `tracing` instrumentation into the code based on a feature flag "tracing".

## Output of `cargo expand`

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use shim_instrument::shim_instrument;
fn main() {
    let a = f1(1);
    let b = f2("world");
    {
        ::std::io::_print(format_args!("{0} {1}\n", a, b));
    };
}
fn f1(a: i32) -> i32 {
    a + 1
}
fn f2(a: &str) -> String {
    {
        let res = ::alloc::fmt::format(format_args!("Hello, {0}", a));
        res
    }
}
```

## Output of `cargo expand --features tracing`

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use shim_instrument::shim_instrument;
fn main() {
    let a = f1(1);
    let b = f2("world");
    {
        ::std::io::_print(format_args!("{0} {1}\n", a, b));
    };
}
fn f1(a: i32) -> i32 {
    {}
    #[allow(clippy::suspicious_else_formatting)]
    {
        let __tracing_attr_span;
        ...
    }
    ...
}
```