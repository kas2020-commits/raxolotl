#[cfg(test)]
extern crate raxolotl;

fn calc_real_sol
(h: f32, a: f32, b: f32, f: impl Fn(f32) -> f32) -> Vec<f32> {
    let n = ((b - a) / h) as usize + 1;
    let mut x = a;
    let mut ans = Vec::with_capacity(n);
    for _i in 0..n {
        ans.push(f(x));
        x += h;
    }
    ans
}

fn check_vec_tol
(v1: &Vec<f32>, v2: &Vec<f32>, tol: f32) -> bool {
    assert!(v1.len() == v2.len());
    let len = v1.len();
    for i in 0..len {
        if (v1[i] - v2[i]).abs() < tol {
            continue;
        } else {
            return false;
        }
    }
    true
}

use num::complex::Complex;
fn check_complex_tol
(v1: &[Complex<f64>], v2: &[Complex<f64>], tol: f64) -> bool {
    assert!(v1.len() == v2.len());
    let len = v1.len();
    for i in 0..len {
        if (v1[i].re - v2[i].re).abs() < tol
        && (v1[i].im - v2[i].im).abs() < tol {
            continue;
        } else {
            return false;
        }
    }
    true
}

#[test]
fn smoke_numerical() {
    type Num = f64;
    let f = |x: Num| { x.powf(3.0) - x - 2.0 };
    let df = |x: Num| { (3.0 * x.powf(2.0)) - 1.0 };
    let tol = 1e-7;
    let max_it = 1000;
    let nm = (
        raxolotl::root_newton_raphson(tol, max_it, &f, &df, 2.0),
        raxolotl::root_bisection(tol, max_it, &f, 0.0, 2.0),
        raxolotl::root_secent(tol, max_it, &f, 0.0, 2.0)
    );
    let answer = 1.5213797;
    assert!((nm.0.0 - answer).abs() < 1e-7);
    assert!((nm.1.0 - answer).abs() < 1e-7);
    assert!((nm.2.0 - answer).abs() < 1e-7);
}

#[test]
fn smoke_integral() {
    type Num = f64;
    let max_it = 1000;
    let tol = 5.0;
    let a = 0.0;
    let b = 10.0;
    let f2 = |x: Num| { x.exp() };
    let nm = (
        raxolotl::composite_trapezoid(max_it, &f2, a, b),
        raxolotl::composite_midpoint(max_it, &f2, a, b),
        raxolotl::composite_simpson(max_it, &f2, a, b),
    );
    let answer = 22025.46579;
    assert!((nm.0 - answer).abs() < tol);
    assert!((nm.1 - answer).abs() < tol);
    assert!((nm.2 - answer).abs() < tol);
}

#[test]
fn smoke_matrix() {
    let mat1 = raxolotl::Matrix::new(
        vec![vec![1.0, 2.0, 8.0], vec![3.0, 4.0, 6.0]]);
    assert!(mat1.at(1,2) == 6.0);
}

#[test]
fn smoke_ode() {
    type Num = f32;
    // closures
    let odf = |x: Num| { 1.0 * x };
    let fc = |x: Num| { x.exp() };
    // calculate the control vector as well as testings
    let control = calc_real_sol(0.01, 0.0, 1.0, fc);
    let r_euler = raxolotl::odesolve_euler(0.01, 0.0, 1.0, 1.0, odf);
    let r_trap  = raxolotl::odesolve_trapezoidal(0.01, 0.0, 1.0, 1.0, odf);
    // asserts
    assert!(check_vec_tol(&control, &r_euler, 0.1));
    assert!(check_vec_tol(&control, &r_trap, 0.1));
}

#[test]
fn smoke_fourier() {
    let input: Vec<_> = [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]
        .iter()
        .map(|x| num::Complex::from(x))
        .collect();
    let output = raxolotl::ifft(&raxolotl::fft(&input));
    assert!(check_complex_tol(&input, &output, 0.001));
}

