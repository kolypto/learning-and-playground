#![no_std]
#![feature(impl_trait_in_assoc_type)]

pub mod wifi;

// Use mk_static!() macro to do esp_radio::init() with a static lifetime.
// The StaticCell crate is useful when you need to initialize a variable at runtime
// but require it to have a static lifetime.
// We will define a macro to create globally accessible static variables.
// Args:
// - Type of variable
// - The value to initialize it with
// The uninit function provides a mutable reference to the uninitialized memory, and we write the value into it.
//
// NOTE: the nightly version of the package contains mk_static!()
#[macro_export]
macro_rules! make_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
