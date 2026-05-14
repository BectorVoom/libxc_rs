//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1088/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1088<F: Float>(t101355: F, t101593: F, t105223: F, t105232: F, t105240: F, t105445: F, t105449: F, t105453: F, t108342: F, t108361: F, t108378: F, t108412: F, t108430: F, t108448: F, t1528: F, t2053: F, t2054: F, t21049: F, t26700: F, t26713: F, t29060: F, t29080: F, t40890: F, t4147: F, t5637: F, t68322: F, t85101: F, t855: F, t86870: F, t86903: F, t86911: F, t98117: F, t98322: F) -> (F,) {
    let t108451 = 12.0 * t4147 * t29080 - t68322 * t2054 - 3.0 * t101355 * t1528 + 6.0 * t4147 * t29060 - 3.0 * t101593 * t1528 + 6.0 * t26713 * t5637 + 6.0 * t26700 * t5637 + 0.15626873635058151147e0 * t86911 - 0.76763589786250567036e0 * t86903 - 0.31253747270116302294e0 * t86870 - t85101 + t108342 + 0.29608813203268075857e0 * t105449 + 0.46058153871750340221e0 * t98117 + t108378 + t108448 + t108430 - 0.9869604401089358619e-1 * t105445 + t108361 - 0.3289868133696452873e-1 * t105453 + 0.29608813203268075857e0 * t105223 + 0.9869604401089358619e-1 * t105240 - 0.39478417604357434476e0 * t105232 + t108412 + 0.49348022005446793095e-1 * t98322 + 24.0 * t855 * t40890 * t2053 * t21049;
    (t108451,)
}
