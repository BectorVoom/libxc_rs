//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 724/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk724<F: Float>(t210: F, t214: F, t6330: F, t6347: F, t1315: F, t3725: F, t3731: F, t3733: F, t3751: F, t5192: F, t5203: F, t562: F, t1807: F, t1834: F, t119: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6353 = t210 * t214 * t6330;
    let t6358 = t210 * t214 * t6347;
    let t6361 = t3725 + 0.77777777777777777775e-2 * t5192 + t3731 + 0.49999999999999999998e-2 * t3733 * t6353 + 0.16666666666666666666e-2 * t5203 - 0.16666666666666666666e-2 * t1315 * t6358 - t3751;
    let t6362 = t6361 * t562;
    let t6364 = t1807 * t1834;
    let t6370 = t119 * t6330;
    let t6371 = t210 * t6370;
    let t6374 = t119 * t6347;
    let t6375 = t210 * t6374;
    let t6378 = t6361 * t225;
    (t6353, t6358, t6361, t6362, t6364, t6370, t6371, t6375, t6378)
}
