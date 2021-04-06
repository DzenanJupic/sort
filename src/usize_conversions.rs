pub trait UsizeConversions {
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
