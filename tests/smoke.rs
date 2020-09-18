mod findroot;
mod integrals;
mod matrix;
mod ode;

fn numerical_test_harness
() {
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
}

fn integral_test_harness
() {
    let max_it = 1000;
    let a = 0f64;
    let b = 10f64;
    let f2 = |x: f64| { x.exp() };
    println!("Trapezoid method: {}", integrals::composite_trapezoid(max_it, &f2, a, b));
    println!("Midpoint method: {}", integrals::composite_midpoint(max_it, &f2, a, b));
    println!("Simpson method: {}", integrals::composite_simpson(max_it, &f2, a, b));
}

fn matrix_test_harness
() {
    let mat1 = matrix::Matrix::new(vec![vec![1f64, 2f64], vec![3f64, 4f64]]);
    mat1.print();
    println!("The item in position [1][1] is {}", mat1.at(1,1));
}

fn print_vec_result
(data: Vec<f64>, x0: f64, h: f64) {
    let mut x = x0;
    for i in data.iter() {
        println!("At x = {}, y = {}", x, i);
        x += h;
    }
}

fn ode_test_harness
() {
    let k = 1f64;
    let odf = |x: f64| { k * x };
    let h = 0.1f64;
    let a = 0f64;
    let b = 1f64;
    let y0 = 1f64;
    // euler's
    let results_euler = ode::euler(h, a, b, y0, odf);
    println!("Euler's method:");
    print_vec_result(results_euler, a, h);
    // trap
    let results_trap = ode::trapezoidal(h, a, b, y0, odf);
    println!("Trapezoidal method:");
    print_vec_result(results_trap, a, h);
}

