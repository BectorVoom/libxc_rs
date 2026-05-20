//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 935/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk935<F: Float>(t22893: F, t6969: F, t22892: F, t3787: F, t6604: F, t22751: F, t6892: F, t6883: F, t6908: F, t22674: F, t6891: F, t1988: F, t22716: F) -> (F, F, F, F, F, F) {
    let t22894 = t22893 * t6969;
    let t22895 = t22892 * t22894;
    let t22897 = t6604 * t3787;
    let t22907 = t22751 * t6892;
    let t22909 = t6883 * t6908;
    let t22920 = t22674 * t6891;
    let t22921 = t22892 * t22920;
    let t22923 = t22716 * t1988;
    (t22895, t22897, t22907, t22909, t22921, t22923)
}
