//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1040/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1040<F: Float>(t12020: F, t8800: F, t115397: F, t115409: F, t115415: F, t115423: F, t115430: F, t117193: F, t117209: F, t117210: F, t117246: F, t122451: F, t122457: F, t122460: F, t122462: F, t122467: F, t122483: F, t122488: F, t1336: F, t1814: F, t1825: F, t32137: F, t32148: F, t33839: F, t3777: F, t5234: F) -> (F, F) {
    let t124223 = t12020 * t8800;
    let t124245 = F::cast_from(0.19739208802178717238e0_f64) * t122451 - F::cast_from(0.3289868133696452873e-1_f64) * t122457 + F::cast_from(0.16449340668482264365e-1_f64) * t122460 + F::cast_from(0.76763589786250567037e-1_f64) * t122462 - F::cast_from(0.3289868133696452873e-1_f64) * t122467 - t117193 + F::cast_from(0.15352717957250113407e0_f64) * t115397 + F::cast_from(0.3289868133696452873e-1_f64) * t115409 + F::cast_from(0.3289868133696452873e-1_f64) * t122483 + F::cast_from(0.76763589786250567037e-1_f64) * t115415 - F::cast_from(0.3289868133696452873e-1_f64) * t122488 + t1814 * t32148 + F::cast_from(0.16449340668482264365e-1_f64) * t115423 - F::cast_from(0.76763589786250567037e-1_f64) * t115430 + t117209 + t117210 - t5234 * t32137 - t3777 * t33839 - t1336 * t117246 * t1825;
    (t124223, t124245)
}
