//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2466/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2466<F: Float>(t1040: F, t21482: F, t10876: F, t21396: F, t248: F, t3101: F, t1041: F, t21138: F, t3051: F, t10403: F, t10408: F, t1046: F, t14211: F, t17607: F, t18014: F, t3071: F, t42388: F, t43361: F, t4338: F, t4343: F, t4636: F, t49743: F, t5873: F, t5880: F, t61675: F, t62079: F, t70106: F) -> F {
    let t70153 = t21482 * t1040;
    let t70162 = t10876 * t248 * t3101 * t21396;
    let t70166 = t1041 * t248 * t3051 * t21138;
    let t70189 = t70153 * t1046 / F::cast_from(4608.0_f64) + t49743 * t5880 / F::cast_from(192.0_f64) + t17607 * t4636 / F::cast_from(1536.0_f64) - t70162 / F::cast_from(768.0_f64) + t70166 / F::cast_from(1152.0_f64) + t10403 * t3071 * t14211 * t70106 / F::cast_from(384.0_f64) - t10403 * t3071 * t5873 * t4343 / F::cast_from(384.0_f64) + t42388 * t3071 * t62079 * t18014 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t10403 * t10408 * t5873 * t4338 - t43361 * t3071 * t5873 * t18014 / F::cast_from(256.0_f64) - t61675 / F::cast_from(144.0_f64);
    t70189
}
