//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 488/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk488<F: Float>(t2764: F, t273: F, t241: F, t63: F, t281: F, t283: F, t699: F, t909: F, t976: F, t891: F, t275: F, t290: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2810 = F::cast_from(0.39862222222222222223e0_f64) * t2764;
    let t2815 = F::cast_from(1.0_f64)/F::sqrt(t273);
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    let t2823 = F::cast_from(0.13692777777777777778e0_f64) * t2822;
    let t2824 = t699 * t909;
    let t2826 = t241 * t976;
    let t2840 = t891 * t891;
    let t2841 = F::cast_from(1.0_f64) / t2840;
    let t2842 = t275 * t2841;
    let t2843 = t290 * t290;
    (t2810, t2815, t2820, t2822, t2823, t2824, t2826, t2842, t2843)
}
