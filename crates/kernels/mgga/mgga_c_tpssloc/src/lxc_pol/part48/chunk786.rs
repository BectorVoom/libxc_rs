//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 786/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk786<F: Float>(t7254: F, t8301: F, t2240: F, t3701: F, t7216: F, t2039: F, t7408: F, t645: F, t8513: F, t8824: F, t31: F, t63: F, t607: F, t8308: F, t79: F, t641: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31867 = t8301 * t7254;
    let t31868 = t2240 * t31867;
    let t32193 = t3701 * t7216;
    let t32318 = t7408 * t2039;
    let t32328 = t8513 * t8824 * t645;
    let t32331 = t63 * t31;
    let t32332 = t32331 * t607;
    let t32333 = t8308 * t32332;
    let t32338 = t79 * t63;
    let t32340 = t8513 * t32338 * t641;
    (t31867, t31868, t32193, t32318, t32328, t32331, t32333, t32338, t32340)
}
