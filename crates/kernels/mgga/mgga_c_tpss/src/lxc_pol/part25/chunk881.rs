//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 881/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk881<F: Float>(t1369: F, t8176: F, t3618: F, t8167: F, t2158: F, t339: F, t790: F, t3632: F, t2383: F, t3685: F, t2169: F, t3667: F, t1381: F, t8286: F, t3590: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t10635 = t8176 * t1369;
    let t10642 = 7.0 / 24.0 * t8167 * t3618;
    let t10652 = t339 * t2158 * t790;
    let t10654 = 7.0 / 1152.0 * t10652 * t3632;
    let t10661 = 35.0 / 576.0 * t2383 * t3685;
    let t10678 = 7.0 / 2304.0 * t2169 * t3667;
    let t10679 = t8286 * t1381;
    let t10684 = t3590 * t72;
    (t10635, t10642, t10654, t10661, t10678, t10679, t10684)
}
