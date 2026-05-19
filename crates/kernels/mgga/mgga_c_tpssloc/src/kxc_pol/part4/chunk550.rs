//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 550/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk550<F: Float>(t888: F, t892: F, t287: F, t891: F, t275: F, t273: F, t276: F, t2764: F, t241: F, t63: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2787 = t888 * t892;
    let t2790 = t891 * t287;
    let t2791 = F::new(1.0) / t2790;
    let t2792 = t275 * t2791;
    let t2798 = F::new(1.0) / t276 / t273;
    let t2802 = F::new(4.0) / F::new(9.0) * t2764;
    let t2810 = F::cast_from(0.39862222222222222223e0_f64) * t2764;
    let t2815 = F::new(1.0)/F::sqrt(t273);
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    (t2787, t2791, t2792, t2798, t2802, t2810, t2815, t2820, t2822)
}
