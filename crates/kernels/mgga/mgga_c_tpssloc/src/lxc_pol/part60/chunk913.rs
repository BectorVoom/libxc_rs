//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 913/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk913<F: Float>(t229: F, t268: F, t6559: F, t225: F, t23228: F, t2056: F, t40772: F, t25: F, t1408: F, t2752: F, t1519: F, t213: F) -> (F, F, F, F, F, F) {
    let t81651 = t6559 * t229 * t268;
    let t82074 = t23228 * t225;
    let t84766 = t2056 * t40772;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    let t86873 = t213 * t1519 * t225;
    (t81651, t82074, t84766, t86716, t86721, t86873)
}
