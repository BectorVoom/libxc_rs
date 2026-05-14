//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1094/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1094<F: Float>(t12012: F, t12147: F, t12157: F, t12160: F, t12161: F, t12164: F, t1345: F, t1347: F, t1348: F, t16186: F, t1995: F, t225: F, t3719: F, t3734: F, t3839: F, t3843: F, t3844: F, t3847: F, t39622: F, t39892: F, t40026: F, t40210: F, t40211: F, t40213: F, t40214: F, t40217: F, t40218: F, t40220: F, t40235: F, t40253: F, t5278: F, t546: F, t548: F) -> (F,) {
    let t40270 = -(t40210 + t40211 + t40213 + t40214 + t40217 + t40218 + t40220 + t40235) * t225 * t548 + 12.0 * t12147 * t1348 - 72.0 * t3839 * t3844 + 18.0 * t3839 * t3847 + 240.0 * t1345 * t12157 - 144.0 * t16186 * t12161 + 12.0 * t1345 * t12164 - 360.0 * t546 * t40253 * t40026 + 360.0 * t5278 * t1995 * t3734 * t3719 - 36.0 * t546 * t3843 * t39622 - 48.0 * t5278 * t12160 * t12012 + 3.0 * t546 * t1347 * t39892;
    (t40270,)
}
