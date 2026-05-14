//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1160/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1160<F: Float>(t3610: F, t86323: F, t3624: F, t11553: F, t2121: F, t2123: F, t2157: F, t43706: F, t1453: F, t81439: F, t1408: F, t2752: F, t193: F, t201: F, t7540: F, t25345: F, t82038: F) -> (F, F, F, F, F, F, F, F) {
    let t86324 = t3610 * t86323;
    let t86327 = t3624 * t86323;
    let t86451 = 0.30461741978670859935e-2 * t2121 * t11553 * t2123;
    let t86524 = t2157 * t43706;
    let t86586 = t81439 * t1453;
    let t86721 = t2752 * t1408;
    let t86736 = t193 * t201 * t7540;
    let t86870 = t82038 * t25345;
    (t86324, t86327, t86451, t86524, t86586, t86721, t86736, t86870)
}
