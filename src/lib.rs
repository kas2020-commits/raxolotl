// #![cfg_attr(test, allow(dead_code))]
#![warn(
    // missing_docs,
    // missing_debug_implementations,
    // missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod findroot;
mod integrals;
mod matrix;
mod ode;

pub use findroot::{
    newton_raphson, bisection, secent,
};

pub use integrals::{
    composite_midpoint, composite_trapezoid, composite_simpson,
};

pub use ode::{
    euler, trapezoidal
};

pub use matrix::Matrix;

