pub use paste::paste;

#[macro_export]
macro_rules! export_days_impl {
    ($( $day:tt),*) => {
        $crate::macros::paste! {
            $( pub use [<day $day>];)*
        }
    };
}

#[macro_export]
macro_rules! export_days {
    () => {
        $crate::macros::export_days_impl!(
            01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24
        );
    };
}

pub use export_days;
pub use export_days_impl;
