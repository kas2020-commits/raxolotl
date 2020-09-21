// #![cfg_attr(test, allow(dead_code))]
#![warn(
    missing_docs,
    // missing_debug_implementations,
    // missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! raxolotl is a numerical computation library designed as a learning tool for
//! common computational methods and their applications. Although the design is
//! centered around teaching applications, the primary focus in the code is on
//! speed and performance. Readability of the code is not the primary focus at
//! any stage of the codebase. A great example is using bitwise shift operations
//! in favor of dividing or multiplying by powers of 2.

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
    euler, trapezoidal, calc_real_sol, check_vec_tol
};

pub use matrix::Matrix;

