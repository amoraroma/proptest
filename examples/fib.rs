//-
// Copyright 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate proptest;
use proptest::prelude::*;

// The worst possible way to calculate Fibonacci numbers
fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        // Setting both fork and timeout is redundant since timeout implies
        // fork, but both are shown for clarity.
        fork: true,
        timeout: 1000,
        .. ProptestConfig::default()
    })]

    // NB We omit #[test] on the test function so that main() can call it.
    fn test_fib(n in prop::num::u64::ANY) {
        // For large n, this will variously run for an extremely long time,
        // overflow the stack, or panic due to integer overflow.
        assert!(fib(n) >= n);
    }
}

fn main() {
    test_fib();
}
