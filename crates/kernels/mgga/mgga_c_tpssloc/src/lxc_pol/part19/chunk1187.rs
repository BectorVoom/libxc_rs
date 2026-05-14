//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1187/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1187<F: Float>(t270: F, t276: F, t39267: F, t2799: F, t2807: F, t2798: F, t273: F, t2815: F, t10588: F, t896: F, t10595: F, t10599: F, t41654: F, t242: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41935 = 1.0 / t276 / t39267 / t270 / 96.0;
    let t41936 = t2799 * t2799;
    let t41937 = t41935 * t41936;
    let t41939 = t2807 * t2807;
    let t41940 = t2798 * t41939;
    let t41942 = f64::powf(t273, -0.25e1);
    let t41943 = t41942 * t41936;
    let t41945 = t2815 * t41939;
    let t41948 = t2798 * t10588 * t896;
    let t41951 = t2815 * t10588 * t896;
    let t41954 = t10595 * t2799 * t2807;
    let t41957 = t10599 * t2799 * t2807;
    let t41959 = 0.31310740740740740741e1 * t41654;
    let t41961 = t281 * t242 * t283;
    (t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957, t41959, t41961)
}
