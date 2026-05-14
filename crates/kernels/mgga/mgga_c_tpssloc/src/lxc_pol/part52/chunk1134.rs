//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1134/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1134<F: Float>(t114172: F, t22892: F, t6891: F, t22573: F, t8449: F, t31220: F, t532: F, t1862: F, t8308: F, t113875: F, t31867: F, t9239: F, t31863: F, t9231: F, t131: F, t8662: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114316 = t22892 * t114172 * t6891;
    let t114360 = t8449 * t22573;
    let t114418 = t532 * t31220;
    let t115833 = t8308 * t1862;
    let t115903 = t113875 * t1862;
    let t116082 = t9239 * t31867;
    let t116106 = t9239 * t31863;
    let t116111 = t9231 * t31863;
    let t116114 = t8662 * t131;
    (t114316, t114360, t114418, t115833, t115903, t116082, t116106, t116111, t116114)
}
