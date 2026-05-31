//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2314/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2314<F: Float>(t23384: F, t28618: F, t28671: F, t82736: F, t100326: F, t100334: F, t14651: F, t1599: F, t25479: F, t25535: F, t3186: F, t3188: F, t6687: F, t7620: F, t82809: F, t89243: F, t89421: F, t89429: F, t89431: F, t89445: F, t89501: F) -> F {
    let t100378 = t23384 * t28618;
    let t100390 = t82736 * t28671;
    let t100396 = -F::cast_from(0.18277045187202515961e-2_f64) * t100378 + F::cast_from(2.0_f64) * t14651 * t7620 + t89421 - t89429 - F::cast_from(0.36554090374405031923e-2_f64) * t89431 + F::cast_from(2.0_f64) * t3186 * t100326 * t3188 - F::cast_from(0.18277045187202515961e-2_f64) * t82809 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t25535 - F::cast_from(0.27415567780803773942e-2_f64) * t100390 - t89445 - F::cast_from(0.16449340668482264365e-1_f64) * t89243 * t25479 - F::cast_from(0.3289868133696452873e-1_f64) * t100334 * t89501;
    t100396
}
