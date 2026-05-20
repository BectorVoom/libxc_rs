//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1834/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1834<F: Float>(t2144: F, t3493: F, t1246: F, t3620: F, t7376: F, t7375: F, t23598: F, t50: F, t131: F, t467: F) -> (F, F, F, F, F, F) {
    let t24803 = t2144 * t3493;
    let t24804 = t24803 * t1246;
    let t24806 = t3620 * t7376;
    let t24807 = t7375 * t24806;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    (t24804, t24806, t24807, t24810, t24811, t24812)
}
