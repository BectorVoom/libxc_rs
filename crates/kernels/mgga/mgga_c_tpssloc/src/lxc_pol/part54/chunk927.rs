//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 927/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk927<F: Float>(t22893: F, t6969: F, t22892: F, t3787: F, t6604: F, t22751: F, t6892: F, t6883: F, t6908: F, t22674: F, t6891: F, t1988: F, t22716: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22894 = t22893 * t6969;
    let t22895 = t22892 * t22894;
    let t22896 = F::new(0.16449340668482264365e-1) * t22895;
    let t22897 = t6604 * t3787;
    let t22907 = t22751 * t6892;
    let t22908 = F::new(0.76763589786250567036e-1) * t22907;
    let t22909 = t6883 * t6908;
    let t22910 = F::new(0.38381794893125283518e-1) * t22909;
    let t22920 = t22674 * t6891;
    let t22921 = t22892 * t22920;
    let t22922 = F::new(0.16449340668482264365e-1) * t22921;
    let t22923 = t22716 * t1988;
    (t22895, t22896, t22897, t22907, t22908, t22909, t22910, t22921, t22922, t22923)
}
