//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 724/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk724<F: Float>(t1023: F, t248: F, t3101: F, t1020: F, t1017: F, t1030: F, t1015: F, t1012: F, t1009: F, t990: F, t1011: F, t1019: F) -> (F, F, F, F, F, F, F, F) {
    let t3103 = t248 * t3101 * t1023;
    let t3104 = t1020 * t3103;
    let t3107 = t1030 * t1017;
    let t3108 = t1015 * t3107;
    let t3109 = t1012 * t3108;
    let t3112 = t990 * t1009;
    let t3113 = t3112 * t1011;
    let t3114 = t3113 * t1019;
    (t3103, t3104, t3107, t3108, t3109, t3112, t3113, t3114)
}
