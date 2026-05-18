//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 668/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk668<F: Float>(t562: F, t6361: F, t1807: F, t1834: F, t119: F, t6330: F, t210: F, t6347: F, t225: F) -> (F, F, F, F, F) {
    let t6362 = t6361 * t562;
    let t6364 = t1807 * t1834;
    let t6370 = t119 * t6330;
    let t6371 = t210 * t6370;
    let t6374 = t119 * t6347;
    let t6375 = t210 * t6374;
    let t6378 = t6361 * t225;
    (t6362, t6364, t6371, t6375, t6378)
}
