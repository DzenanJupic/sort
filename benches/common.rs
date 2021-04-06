#[allow(unused_macros)]
macro_rules! criterion_sort_benchmark {
    (
        $(
            $fn:ident <$($ty:ty),+> up to $pow:literal
        )*
    ) => {
        mod benches {
            use ::std::hint::black_box;
            use ::std::default::Default;
            use ::std::clone::Clone;
            
            use ::rand::{Fill, thread_rng, Rng};
            use ::criterion::Criterion;
            
            fn rand_vec<T>(len: usize) -> Vec<T>
                where
                    T: Default + Clone,
                    [T]: Fill {
                let mut vec = vec![T::default(); len];
                thread_rng().fill(vec.as_mut_slice());
                vec
            }
            
            $(
                use super::$fn;
            
                $(
                    ::concat_idents::concat_idents!(fn_name = $fn, _, $ty, _benchmark {
                        fn fn_name(c: &mut Criterion) {
                            c.bench_function(
                                concat!(stringify!($fn), " [", stringify!($ty), "; 0]"),
                                |b| b.iter(|| $fn::<$ty>(black_box(&mut rand_vec( 0 ))))
                            );
                            
                            ::seq_macro::seq!( N in 0..=$pow {
                                c.bench_function(
                                    concat!(stringify!($fn), " [", stringify!($ty), "; ", " 2^", stringify!(N), "]"),
                                    |b| b.iter(|| $fn::<$ty>(black_box(&mut rand_vec( 2usize.pow(N) ))))
                                );
                            });
                        }
                    });
                )+
            
                ::concat_idents::concat_idents!(bench_group = $fn, _bench_group {
                    pub fn bench_group() {
                        let mut criterion = ::criterion::Criterion::default()
                            .configure_from_args();
                        
                        $(
                            ::concat_idents::concat_idents!(fn_name = $fn, _, $ty, _benchmark {
                                fn_name(&mut criterion);
                            });
                        )+
                    }
                });                    
            )*
        }
        
        fn main() {
            ::criterion::__warn_about_html_reports_feature();
            ::criterion::__warn_about_cargo_bench_support_feature();

            $(
                ::concat_idents::concat_idents!(bench_group = $fn, _bench_group {
                    benches::bench_group();
                });
            )*

            ::criterion::Criterion::default()
                .configure_from_args()
                .final_summary();
        }
    };
}
