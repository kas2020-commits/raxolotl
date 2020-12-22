#[cfg(test)]
extern crate raxolotl;
type Num = raxolotl::Num;

fn check_vec_tol
(v1: &Vec<Num>, v2: &Vec<Num>, tol: Num) -> bool {
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

fn check_odesolver
(tspan: &[Num], f: impl Fn(Num) -> Vec<Num>, v2: &Vec<Vec<Num>>, tol: Num) -> bool {
    let n = tspan.len();
    for i in 0..n {
        if check_vec_tol(&f(tspan[i]), &v2[i], tol) == false {
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
fn smoke_ode() {
    // closures
    let odf = |_t: Num, x: &[Num]| { vec![1.0 * x[0]] };
    let fc = |x: Num| { vec![x.exp()] };
    let tspan = raxolotl::linspace(0., 1., 100);
    let y0 = vec![1.];
    let r_euler = raxolotl::odesolve_euler(&tspan, &y0, odf);
    assert!(check_odesolver(&tspan, fc, &r_euler, 0.1));
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
fn smoke_fourier() {
    let input: Vec<_> = [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]
        .iter()
        .map(|x| num::Complex::from(x))
        .collect();
    let output = raxolotl::ifft(&raxolotl::fft(&input));
    assert!(check_complex_tol(&input, &output, 0.001));
}

