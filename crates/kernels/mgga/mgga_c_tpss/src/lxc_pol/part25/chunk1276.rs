//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1276/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1276<F: Float>(t61062: F, t764: F, t238: F, t5543: F, t1695: F, t212: F, t60720: F, t2376: F, t339: F, t5557: F, t803: F, t228: F, t32386: F) -> (F, F, F, F, F, F) {
    let t61063 = t61062 * t764;
    let t61072 = t5543 * t238;
    let t61079 = t60720 * t212 * t1695;
    let t61086 = t339 * t5557 * t2376;
    let t61087 = t61086 * t803;
    let t61195 = t32386 * t228;
    (t61063, t61072, t61079, t61086, t61087, t61195)
}
