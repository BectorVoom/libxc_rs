//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2278/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2278<F: Float>(t24637: F, t8009: F, t24588: F, t8020: F, t1184: F, t4929: F, t1715: F, t3469: F, t24645: F, t7999: F, t1186: F, t1235: F, t15789: F, t1716: F, t1761: F, t24567: F, t24571: F, t24589: F, t24605: F, t24611: F, t24615: F, t27406: F, t27411: F, t27437: F, t27445: F, t27453: F, t27799: F, t460: F, t7283: F, t7286: F, t7300: F, t86403: F, t86475: F) -> (F, F, F, F) {
    let t94391 = t8009 * t24637;
    let t94395 = t8020 * t24588;
    let t94400 = t4929 * t1184;
    let t94404 = t1715 * t3469;
    let t94427 = F::cast_from(0.14621636149762012769e-1_f64) * t7999 * t24645;
    let t94428 = -F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t1716 * t24611 - t86475 * t1761 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t94391 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t24605 + F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t24571 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t94400 * t27799 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t94404 * t27799 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t27453 * t460 * t1235 * t7286 + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t24567 * t27411 + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t7300 * t24615 * t15789 + F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t86403 * t27445 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t86403 * t27437 - t94427;
    (t94395, t94400, t94404, t94428)
}
