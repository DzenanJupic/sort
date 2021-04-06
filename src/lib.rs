#![feature(specialization, const_trait_impl)]
#![allow(incomplete_features)]

pub use crate::counting_sort::CountingSort;
pub use crate::usize_conversions::UsizeConversions;

// Comparison sorts
// todo
pub mod bubble_sort;
// todo
pub mod heap_sort;
// todo
pub mod insertion_sort;
// todo
pub mod merge_sort;
// todo
pub mod quick_sort;

// Non-comparison sorts
#[doc(hidden)]
pub mod counting_sort;
// todo
pub mod radix_sort;

#[doc(hidden)]
pub mod usize_conversions;

/// A trait for sortable types.
/// 
/// This trait is the heart piece of the `sort` crate and provides an interface for sorting all
/// kinds of types. Note that this trait gives no guarantees whether or not the underlying algorithm
/// is stable. If you need a stable sorting algorithm, have a look at [`SortStable`] instead. 
/// 
/// By default it actually uses the `std` library's `<[T]>::sort` method for sorting. But there's
/// one major difference: `Sort` will try to find a faster and better implementation for the type
/// you want to sort. Without any runtime overhead!
/// 
/// It does so by using (the currently unstable) specialization feature, that allows to override
/// trait implementation with more specific ones.
/// As said, by default `Sort` just falls back to `<[T]>::sort` as long as `T` implements [`Ord`].
/// But when for example when sorting booleans, there's no need to even compare anything! You can
/// just use [`CountingSort`](counting_sort::CountingSort) instead. Counting sort is an algorithm, 
/// that is able to sort small types, like `bool` or `u8` magnitudes faster then a normal sort 
/// algorithm. But since it's not applicable for bigger types, like `String`, it's usually not used. 
pub trait Sort {
    /// A sort implantation that gives no guarantees about how sorting works. 
    fn sort(&mut self);
}

/// A trait that acts as a trusted contract that the provided [`SortStable::sort_stable`]
/// implementation is actually stable.
/// 
/// When not required, it's usually recommended to not force stable sorting by using [`Sort`] 
/// instead of [`SortStable`], since stable sorting often comes with a performance and memory 
/// overhead.
/// 
/// Being stable means that when you sort a list of elements, equal elements will keep their 
/// relative position to each other:
/// 
/// ![stable_vs_unusable](https://upload.wikimedia.org/wikipedia/commons/thumb/8/82/Sorting_stability_playing_cards.svg/330px-Sorting_stability_playing_cards.svg.png)
/// 
/// It's unsafe to implement this trait, since there's not really a way for the compiler to prove 
/// that a sorting algorithm is stable, and certain things might heavily rely on an sorting
/// algorithm being stable. 
pub unsafe trait SortStable: Sort {
    /// A sort implantation that guarantees to be stable. 
    fn sort_stable(&mut self) {
        <Self as Sort>::sort(self)
    }
}

impl<T> Sort for [T]
    where T: Ord {
    #[inline]
    default fn sort(&mut self) {
        if self.len() < 2 { return; }
        <[T]>::sort(self);
    }
}

macro_rules! impl_specific_sort {
    (impl Sort for $ty:ty as $sort_fn:path) => {
        impl $crate::Sort for $ty {
            #[inline]
            fn sort(&mut self) {
                $sort_fn(self);
            }
        }
    };
}

impl_specific_sort!(impl Sort for [bool] as counting_sort::CountingSort::sort);
impl_specific_sort!(impl Sort for [u8] as counting_sort::CountingSort::sort);
impl_specific_sort!(impl Sort for [i8] as counting_sort::CountingSort::sort);
