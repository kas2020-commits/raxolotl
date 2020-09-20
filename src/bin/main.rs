extern crate raxolotl;

fn print_vec_result
(data: Vec<f32>) {
    for i in data.iter() {
        println!("y = {}", i);
    }
}

fn main
() {
    let odf = |x: f32| { 5.0 * x };
    // euler's
    let results_euler = raxolotl::euler(0.1, 0.0, 1.0, 1.0, odf);
    println!("Euler's method:");
    print_vec_result(results_euler);
    // trap
    let results_trap = raxolotl::trapezoidal(0.1, 0.0, 1.0, 1.0, odf);
    println!("Trapezoidal method:");
    print_vec_result(results_trap);
}
