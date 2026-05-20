//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1550/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1550<F: Float>(t16253: F, t16319: F, t16361: F, t16411: F, t553: F, t3901: F, t5287: F, t1352: F, t16036: F, t3856: F, t5348: F, t1834: F, t3787: F) -> (F, F, F, F, F, F) {
    let t16413 = t16253 + t16319 + t16361 + t16411;
    let t16414 = t553 * t16413;
    let t16416 = t3901 * t5287;
    let t16419 = t16036 * t1352;
    let t16423 = t5348 * t3856;
    let t16428 = t3787 * t1834;
    (t16413, t16414, t16416, t16419, t16423, t16428)
}
