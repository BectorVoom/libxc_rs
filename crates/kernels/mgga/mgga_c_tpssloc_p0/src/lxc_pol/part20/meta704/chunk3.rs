//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2677/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2677<F: Float>(t12364: F, t5234: F, t1354: F, t16288: F, t3858: F, t1351: F, t3734: F, t12012: F, t12168: F, t12413: F, t1341: F, t1343: F, t1363: F, t16101: F, t16208: F, t16224: F, t16311: F, t16394: F, t1799: F, t221: F, t3719: F, t3778: F, t3803: F, t3805: F, t3870: F, t40160: F, t5187: F, t5246: F, t5248: F, t5250: F, t5301: F, t53958: F, t54284: F, t54293: F, t54295: F, t54527: F, t820: F) -> F {
    let t54532 = t5234 * t12364;
    let t54533 = t54532 * t1354;
    let t54534 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t54533;
    let t54535 = t16288 * t3858;
    let t54542 = t1351 * t3734;
    let t54552 = -t16394 * t12413 / F::cast_from(1024.0_f64) + t5246 * t5248 * t53958 * t5250 / F::cast_from(512.0_f64) - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t16101 * t221 * t54284 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1363 * t3870 * t820 * t1799 * t12012 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t54293 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t54295 - t3778 * t16208 / F::cast_from(1024.0_f64) - t1341 * t1343 * t820 * t54527 / F::cast_from(3072.0_f64) - t54534 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t54535 + t3803 * t3805 * t5301 * t12168 / F::cast_from(768.0_f64) + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t40160 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t5246 * t16224 * t16311 * t54542 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t1363 * t3870 * t820 * t5187 * t3719;
    t54552
}
