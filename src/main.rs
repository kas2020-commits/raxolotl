mod findroot;
mod integrals;
mod matrix;

fn main
() {
    // numerical section
    let f = |x: f64| { x.powf(3f64) - x - 2f64 };
    let df = |x: f64| { (3f64 * x.powf(2f64)) - 1f64 };
    let tol = 1e-7;
    let max_it = 1000;
    let nm = (
        findroot::newton_raphson(tol, max_it, &f, &df, 2.0),
        findroot::bisection(tol, max_it, &f, 0f64, 2f64),
        findroot::secent(tol, max_it, &f, 0f64, 2f64));
    println!("Newton method: {} after {} itterations", nm.0.0, nm.0.1);
    println!("Bisection method: {} after {} itterations", nm.1.0, nm.1.1);
    println!("Secent method: {} after {} itterations", nm.2.0, nm.2.1);

    // integration section
    let a = 0f64;
    let b = 10f64;
    let f2 = |x: f64| { x.exp() };
    println!("Trapezoid method: {}", integrals::composite_trapezoid(max_it, &f2, a, b));
    println!("Midpoint method: {}", integrals::composite_midpoint(max_it, &f2, a, b));
    println!("Simpson method: {}", integrals::composite_simpson(max_it, &f2, a, b));

    // matrix smoke test
    let mat1 = matrix::Matrix {
        rows: 2, cols: 2, data: vec![vec![1f64, 2f64], vec![1f64, 2f64]]};
    mat1.print();
}

