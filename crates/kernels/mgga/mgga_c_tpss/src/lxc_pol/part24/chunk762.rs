//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 762/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk762<F: Float>(t1062: F, t5145: F, t2957: F, t5129: F, t2961: F, t4044: F, t5066: F, t5070: F, t5074: F, t434: F, t1542: F) -> (F, F, F, F, F) {
    let t5146 = t5145 * t1062;
    let t5149 = t5129 * t2957;
    let t5156 = t2961 - 0.61805555555555555556e-2 * t4044 - 0.61805555555555555555e-2 * t5066 + 0.18541666666666666667e-1 * t5070 + 0.92708333333333333333e-2 * t5074;
    let t5157 = t5156 * t434;
    let t5161 = t1542 * t1542;
    (t5146, t5149, t5156, t5157, t5161)
}
