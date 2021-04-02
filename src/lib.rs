#![feature(specialization, const_trait_impl)]
#![allow(incomplete_features)]

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
pub mod counting_sort;
// todo
pub mod radix_sort;

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


trait UsizeConversions {
    fn into_usize(self) -> usize;
    fn as_usize(&self) -> usize;

    fn from_usize(n: usize) -> Self;
}

macro_rules! impl_usize_conversions {
    ($ty:ty) => {
        impl_usize_conversions!($ty => into: {} from: {}); 
    };
    ($ty:ty as $($as_usize:tt)*) => { 
        impl_usize_conversions!($ty => into: { as $($as_usize)*} from: { as $ty}); 
    };
    ($ty:ty => into: { $($as_usize:tt)* } from:  { $($from_usize:tt)* }) => {
        impl const UsizeConversions for $ty {
            #[inline]
            fn into_usize(self) -> usize {
                self $($as_usize)*
            }
            
            #[inline]
            fn as_usize(&self) -> usize {
                *self $($as_usize)*
            }
            
            #[inline]
            fn from_usize(n: usize) -> Self {
                n $($from_usize)*
            }
        }
    };
}

impl_usize_conversions!(char => into: { as u8 as usize} from: { as u8 as char });

impl_usize_conversions!(u8 as usize);
impl_usize_conversions!(i8 as u8 as usize);
impl_usize_conversions!(u16 as usize);
impl_usize_conversions!(i16 as u16 as usize);
impl_usize_conversions!(u32 as usize);
impl_usize_conversions!(i32 as u32 as usize);
impl_usize_conversions!(u64 as usize);
impl_usize_conversions!(i64 as u64 as usize);
impl_usize_conversions!(usize);
impl_usize_conversions!(isize as usize);
