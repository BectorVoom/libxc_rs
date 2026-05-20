//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1561/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1561<F: Float>(t22751: F, t6892: F, t6883: F, t6908: F, t22674: F, t6891: F, t22892: F, t1988: F, t22716: F, t22724: F, t6898: F, t6902: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t22907 = t22751 * t6892;
    let t22908 = F::cast_from(0.76763589786250567036e-1_f64) * t22907;
    let t22909 = t6883 * t6908;
    let t22910 = F::cast_from(0.38381794893125283518e-1_f64) * t22909;
    let t22920 = t22674 * t6891;
    let t22921 = t22892 * t22920;
    let t22922 = F::cast_from(0.16449340668482264365e-1_f64) * t22921;
    let t22923 = t22716 * t1988;
    let t22925 = t22724 * t6898;
    let t22927 = t794 * t6902;
    (t22908, t22910, t22920, t22922, t22923, t22925, t22927)
}
