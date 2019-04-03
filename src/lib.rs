/*!

[![](https://docs.rs/longest-increasing-subsequence/badge.svg)](https://docs.rs/longest-increasing-subsequence/)
[![](https://img.shields.io/crates/v/longest-increasing-subsequence.svg)](https://crates.io/crates/longest-increasing-subsequence)
[![](https://img.shields.io/crates/d/longest-increasing-subsequence.svg)](https://crates.io/crates/longest-increasing-subsequence)
[![Build Status](https://dev.azure.com/fitzgen/longest-increasing-subsequence/_apis/build/status/fitzgen.longest-increasing-subsequence?branchName=master)](https://dev.azure.com/fitzgen/longest-increasing-subsequence/_build/latest?definitionId=1&branchName=master)

## Longest Increasing Subsequence

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

## API

This crate exposes two functions for finding a longest increasing subsequence
within a slice:

1. The high-level, easy-to-use `lis` function takes any slice of `T: Ord` and
returns the LIS as a vector of indices into that slice.

2. The low-level `lis_with` function takes a custom comparator and lets you
bring your own allocations (which lets you choose to reuse allocations or use a
custom allocator).

Both functions use the same underlying algorithm. They execute in *O(n log n)*
time and use *O(n)* memory.

## Example

```
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

 */

/// The high-level, easy-to-use function for finding a longest increasing
/// subsequence.
///
/// Takes any slice `&[T]` and uses the `T: Ord` implementation to determine the
/// LIS.
///
/// The LIS is returned as a vector of indices into the input items slice.
///
/// # Example
///
/// ```
/// use longest_increasing_subsequence::lis;
///
/// let xs = vec![9, 2, 8, 3, 5];
/// for i in lis(&xs) {
///     println!("{} at index {}", xs[i], i);
/// }
///
/// // Prints:
/// // 2 at index 1
/// // 3 at index 3
/// // 5 at index 4
/// ```
pub fn lis<T>(items: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let mut seq = Vec::new();
    let p = &mut vec![0; items.len()];
    let m = &mut vec![0; items.len()];
    lis_with(items, &mut seq, |a, b| a < b, p, m);
    seq.reverse();
    seq
}

/// The low-level function for finding a longest increasing subsequence.
///
/// This low-level function allows you to:
///
/// * customize the comparator function to something other than `T: Ord`,
///
/// * bring your own allocations for the algorithm's temporary scratch space (so
/// you can reuse the same allocations across multiple `lis_with` calls, or use
/// a custom allocator, etc...),
///
/// * and collect the resulting LIS into a custom collection data structure.
///
/// Note that the `out_seq` is given the indices of the LIS in **reverse order**
/// from the end of the LIS first to the start of the LIS last.
///
/// ## Panics
///
/// Panics if `items`, `predecessors`, and `starts` do not all have the same
/// length.
///
/// ## Example
///
/// ```
/// use longest_increasing_subsequence::lis_with;
/// use std::collections::HashSet;
///
/// // Create allocations for the algorithm's scratch space.
/// let mut predecessors = Vec::new();
/// let mut starts = Vec::new();
///
/// // And a collection to contain the results.
/// let mut results = HashSet::new();
///
/// // A slice whose LIS we would like to find.
/// let xs = vec![9, 2, 8, 3, 5];
///
/// // Ensure our allocations have enough space.
/// predecessors.resize_with(xs.len(), Default::default);
/// starts.resize_with(xs.len(), Default::default);
///
/// lis_with(
///     &xs,
///     &mut results,
///     |a, b| a < b,
///     &mut predecessors,
///     &mut starts,
/// );
///
/// assert_eq!(results, vec![1, 3, 4].into_iter().collect());
///
/// // Another slice whose LIS we would like to find.
/// let ys = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// // We are going to reuse our previous scratch space. Again, ensure we
/// // have enough space.
/// predecessors.resize_with(ys.len(), Default::default);
/// starts.resize_with(ys.len(), Default::default);
///
/// results.clear();
/// lis_with(
///     &ys,
///     &mut results,
///     |a, b| a < b,
///     &mut predecessors,
///     &mut starts,
/// );
///
/// assert_eq!(results, vec![9, 10, 11, 12, 13, 14, 15, 16, 17, 18].into_iter().collect());
/// ```
pub fn lis_with<T, S, F>(
    items: &[T],
    out_seq: &mut S,
    mut less_than: F,
    predecessors: &mut [usize],
    starts: &mut [usize],
) where
    S: Extend<usize>,
    F: FnMut(&T, &T) -> bool,
{
    assert_eq!(items.len(), predecessors.len());
    assert_eq!(predecessors.len(), starts.len());

    if items.is_empty() {
        return;
    }

    unsafe {
        let mut k = 0;
        let len = items.len();

        for i in 0..len {
            let j = *get_unchecked(starts, k);

            if less_than(get_unchecked(items, j), get_unchecked(items, i)) {
                set_unchecked(predecessors, i, j);
                k += 1;
                set_unchecked(starts, k, i);
                continue;
            }

            let mut lo = 0;
            let mut hi = k;

            while lo < hi {
                // Get the mid point while handling overflow.
                let mid = (lo >> 1) + (hi >> 1) + (lo & hi & 1);
                if less_than(
                    get_unchecked(items, *get_unchecked(starts, mid)),
                    get_unchecked(items, i),
                ) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }

            if less_than(
                get_unchecked(items, i),
                get_unchecked(items, *get_unchecked(starts, lo)),
            ) {
                if lo > 0 {
                    set_unchecked(predecessors, i, *get_unchecked(starts, lo - 1));
                }
                set_unchecked(starts, lo, i);
            }
        }

        let u = k + 1;
        let mut v = *get_unchecked(starts, k);
        out_seq.extend((0..u).rev().map(|_| {
            let w = v;
            v = *get_unchecked(predecessors, v);
            w
        }));
    }
}

#[inline(always)]
unsafe fn get_unchecked<T>(slice: &[T], index: usize) -> &T {
    debug_assert!(index < slice.len());
    slice.get_unchecked(index)
}

#[inline(always)]
unsafe fn set_unchecked<T>(slice: &mut [T], index: usize, value: T) {
    debug_assert!(index < slice.len());
    *slice.get_unchecked_mut(index) = value;
}
