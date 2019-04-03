# `longest-increasing-subsequence`


[![](https://docs.rs/longest-increasing-subsequence/badge.svg)](https://docs.rs/longest-increasing-subsequence/)
[![](https://img.shields.io/crates/v/longest-increasing-subsequence.svg)](https://crates.io/crates/longest-increasing-subsequence)
[![](https://img.shields.io/crates/d/longest-increasing-subsequence.svg)](https://crates.io/crates/longest-increasing-subsequence)
[![Build Status](https://dev.azure.com/fitzgen/longest-increasing-subsequence/_apis/build/status/fitzgen.longest-increasing-subsequence?branchName=master)](https://dev.azure.com/fitzgen/longest-increasing-subsequence/_build/latest?definitionId=1&branchName=master)

### Longest Increasing Subsequence

> The longest increasing subsequence problem is to find a subsequence of a given
> sequence in which the subsequence's elements are in sorted order, lowest to
> highest, and in which the subsequence is as long as possible. This subsequence
> is not necessarily contiguous, or unique.

&mdash; [Wikipedia](https://en.wikipedia.org/wiki/Longest_increasing_subsequence)

For example, consider this sequence of integers:

> 2, 9, 4, 7, 3, 4, 5

The longest increasing subsequence (LIS) for this sequence is *2, 3, 4, 5*.

Note that there is not always a *singular* LIS. Consider this sequence:

> 2, 6, 5

In this sequence, both *2, 5* and *2, 6* are LISs.

### API

This crate exposes two functions for finding a longest increasing subsequence
within a slice:

1. The high-level, easy-to-use `lis` function takes any slice of `T: Ord` and
returns the LIS as a vector of indices into that slice.

2. The low-level `lis_with` function takes a custom comparator and lets you
bring your own allocations (which lets you choose to reuse allocations or use a
custom allocator).

Both functions use the same underlying algorithm. They execute in *O(n log n)*
time and use *O(n)* memory.

### Example

```rust
use longest_increasing_subsequence::lis;

let xs = vec![9, 2, 8, 3, 5];
for i in lis(&xs) {
    println!("{} at index {}", xs[i], i);
}

// Prints:
// 2 at index 1
// 3 at index 3
// 5 at index 4
```

