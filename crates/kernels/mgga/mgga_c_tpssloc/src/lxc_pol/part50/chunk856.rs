//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 856/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk856<F: Float>(t25224: F, t6555: F, t6552: F, t1911: F, t4300: F, t2718: F, t1519: F, t828: F, t232: F, t6646: F, t1888: F, t13384: F, t23110: F, t7524: F, t23185: F, t234: F, t6604: F) -> (F, F, F, F, F, F) {
    let t25229 = t25224 * t6555;
    let t25230 = t6552 * t25229;
    let t25232 = t1911 * t4300;
    let t25233 = t2718 * t25232;
    let t25236 = t1519 * t828;
    let t25237 = t25236 * t232;
    let t25238 = t6646 * t25237;
    let t25239 = t1888 * t25238;
    let t25241 = t13384 * t232;
    let t25242 = t6646 * t25241;
    let t25243 = t1888 * t25242;
    let t25245 = t23110 * t7524;
    let t25246 = t23185 * t25245;
    let t25248 = t6604 * t234;
    (t25230, t25233, t25239, t25243, t25246, t25248)
}
