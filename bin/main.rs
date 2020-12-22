extern crate raxolotl;
extern crate num;
type Num = raxolotl::Num;

fn main() {
    let odf = |_t: Num, x: &[Num]| { vec![
        -x[1] - x[2],
        x[0] + (0.2*x[1]),
        0.2 + (x[0]- 5.7)*x[2]
    ]};
    let a = 0.;
    let b = 500.;
    let n = 2500;
    let tspan = raxolotl::linspace(a, b, n);
    let y0 = vec![0., 0., 0.];
    let sol_euler = raxolotl::odesolve_euler(&tspan, &y0, odf);
    let sol_rk4 = raxolotl::odesolve_erk4(&tspan, &y0, odf);
    raxolotl::vec_export_matlab(&tspan, "tspan", "timespan.m");
    raxolotl::mat_export_matlab(&sol_euler, "sol_euler", "solution_euler.m");
    raxolotl::mat_export_matlab(&sol_rk4, "sol_rk4", "solution_rk4.m");
}
