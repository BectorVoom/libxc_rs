//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2724/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2724<F: Float>(t550: F, t56913: F, t3862: F, t6379: F, t5293: F, t53945: F, t19921: F, t3866: F, t19926: F, t12215: F, t12397: F, t12429: F, t1307: F, t1341: F, t1343: F, t1352: F, t1363: F, t16394: F, t16405: F, t19631: F, t19843: F, t19972: F, t19996: F, t20000: F, t210: F, t3733: F, t3734: F, t3783: F, t3803: F, t3870: F, t40025: F, t40282: F, t5248: F, t53990: F, t54162: F, t54582: F, t56817: F, t6370: F, t6374: F, t6422: F, t820: F) -> (F, F) {
    let t57354 = t56913 * t550;
    let t57383 = t6379 * t3862;
    let t57392 = t53945 * t5293;
    let t57396 = t3866 * t19921;
    let t57398 = t3866 * t19926;
    let t57400 = -t12397 * t6422 / F::new(3072.0) - t1341 * t1343 * t820 * t57354 / F::new(1536.0) + F::new(5.0) / F::new(384.0) * t3783 * t19996 + F::new(5.0) / F::new(384.0) * t1363 * t3870 * t820 * t19631 * t1307 + t3733 * t210 * t19843 * t1307 / F::new(8.0) - t53990 * t20000 / F::new(256.0) - t54162 * t5293 / F::new(768.0) + F::new(119.0) / F::new(1728.0) * t40282 + F::new(5.0) / F::new(4.0) * t40025 * t210 * t6370 * t3734 - t12215 * t210 * t6374 * t3734 / F::new(4.0) + F::new(119.0) / F::new(13824.0) * t57383 + F::new(455.0) / F::new(324.0) * t54582 - t3803 * t5248 * t56817 * t1352 / F::new(1536.0) - t12429 * t19972 / F::new(768.0) + F::new(7.0) / F::new(1152.0) * t57392 - F::new(5.0) / F::new(384.0) * t16394 * t16405 + F::new(35.0) / F::new(96.0) * t57396 - F::new(35.0) / F::new(288.0) * t57398;
    (t57354, t57400)
}
