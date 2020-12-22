extern crate raxolotl;
extern crate num;
type Num = f32;

// fn print_vec
// (v1: &[Num]) {
//     for i in v1.iter() {
//         println!("{},", i);
//     }
// }

fn print_mat
(v1: &Vec<Vec<Num>>) {
    for i in v1.iter() {
        for k in i.iter() {
            print!("{},", k);
        }
        println!("");
    }
}

fn linspace
(a: f32, b: f32, n: usize) -> Vec<f32> {
    let mut y = Vec::with_capacity(n);
    let h = (b - a) / n as f32;
    y.push(a);
    for i in 0..n {
        y.push(y[i] + h);
    }
    y
}

fn main() {
    // closures
    let odf = |_t: Num, x: &[Num]| { vec![
        -x[1] - x[2],
        x[0] + (0.2*x[1]),
        0.2 + (x[0]- 5.7)*x[2]
    ]};
    let a = 0.;
    let b = 500.;
    let n = 25000;
    let tspan = linspace(a, b, n);
    let y0 = vec![0., 0., 0.];
    // let r_euler = raxolotl::odesolve_euler(&tspan, &y0, odf);
    let sol = raxolotl::odesolve_erk4(&tspan, &y0, odf);
    // println!("Starting time: {}, End time: {}", tspan[0], tspan[tspan.len()-1]);
    // println!("n: {}, y0: [{}, {}, {}]", n, y0[0], y0[1], y0[2]);
    // println!("Length of tspan: {}", tspan.len());
    // println!("y0 = [");
    // print_vec(&y0);
    // println!("]");
    // println!("tspan = [");
    // print_vec(&tspan);
    // println!("]");
    // println!("Approximation at the start of the timespan");
    // print_vec(&r_euler[0]);
    // print_vec(&r_erk4[0]);
    // println!("Approximation in the middle of the timespan");
    // print_vec(&r_euler[n / 2]);
    // print_vec(&r_erk4[n / 2]);
    // println!("Approximation at the end of the timespan");
    // print_vec(&r_euler[n-1]);
    // print_vec(&r_erk4[n-1]);
    println!("sol = [");
    print_mat(&sol);
    println!("]");
    println!("plot3(sol(:,1), sol(:,2), sol(:,3));");
}
