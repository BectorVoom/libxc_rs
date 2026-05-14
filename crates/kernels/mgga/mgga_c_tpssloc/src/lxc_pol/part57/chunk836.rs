//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 836/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk836<F: Float>(t33272: F, t81228: F, t81326: F, t33240: F, t6883: F, t225: F, t33267: F, t115352: F, t22892: F, t7691: F, t6897: F, t8621: F, t90544: F, t1799: F, t2085: F, t22704: F, t22705: F, t33280: F) -> (F, F, F, F, F, F, F) {
    let t122281 = t81228 * t81326 * t33272;
    let t122295 = t6883 * t33240;
    let t122297 = t33267 * t225;
    let t122331 = t22892 * t115352 * t7691;
    let t122390 = t6897 * t90544 * t8621;
    let t122448 = t2085 * t1799;
    let t122460 = t22704 * t22705 * t33280;
    (t122281, t122295, t122297, t122331, t122390, t122448, t122460)
}
