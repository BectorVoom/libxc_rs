//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 500/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk500<F: Float>(t1932: F, t3131: F, t1014: F, t3032: F, t3031: F, t390: F, t1878: F, t268: F, t405: F, t1091: F, t690: F) -> (F, F, F, F, F, F) {
    let t3188 = t1932 * t3131;
    let t3199 = t3032 * t1014;
    let t3200 = t3031 * t3199;
    let t3215 = t390 * t390;
    let t3216 = F::new(1.0) / t3215;
    let t3236 = t268 * t1878 * t405;
    let t3237 = F::cast_from(0.23744444444444444444e-1_f64) * t3236;
    let t3238 = t690 * t1091;
    (t3188, t3200, t3216, t3236, t3237, t3238)
}
