//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 360/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk360<F: Float>(t1878: F, t268: F, t271: F, t1043: F, t154: F, t632: F, t2289: F, t287: F, t891: F, t275: F, t273: F, t276: F, t241: F, t63: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2764 = t268 * t1878 * t271;
    let t2765 = 0.23744444444444444444e-1 * t2764;
    let t2768 = t154 * t1043;
    let t2769 = t632 * t632;
    let t2770 = 1.0 / t2769;
    let t2775 = 1.0 / t2289;
    let t2790 = t891 * t287;
    let t2791 = 1.0 / t2790;
    let t2792 = t275 * t2791;
    let t2798 = 1.0 / t276 / t273;
    let t2802 = 4.0 / 9.0 * t2764;
    let t2810 = 0.39862222222222222223e0 * t2764;
    let t2815 = 1.0/f64::sqrt(t273);
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    (t2764, t2765, t2768, t2770, t2775, t2792, t2798, t2802, t2810, t2815, t2820, t2822)
}
