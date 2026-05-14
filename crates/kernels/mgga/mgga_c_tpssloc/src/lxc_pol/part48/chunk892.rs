//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 892/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk892<F: Float>(t1873: F, t84078: F, t94165: F, t24462: F, t6534: F, t131: F, t2108: F, t39063: F, t8662: F, t31867: F, t9239: F, t2240: F, t24503: F, t8301: F, t39049: F, t9231: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116004 = 0.135e2 * t84078 * t1873;
    let t116006 = 27.0 * t94165 * t1873;
    let t116008 = 27.0 * t24462 * t6534;
    let t116065 = t2108 * t131;
    let t116075 = t39063 * t8662;
    let t116082 = t9239 * t31867;
    let t116088 = t2240 * t8301 * t24503;
    let t116096 = t39049 * t8662;
    let t116099 = t9231 * t31867;
    (t116004, t116006, t116008, t116065, t116075, t116082, t116088, t116096, t116099)
}
