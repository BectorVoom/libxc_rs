//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2276/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2276<F: Float>(t23384: F, t28510: F, t28565: F, t381: F, t1065: F, t14552: F, t1635: F, t17588: F, t17635: F, t23327: F, t23329: F, t23330: F, t23346: F, t23369: F, t25423: F, t25784: F, t28470: F, t28697: F, t3169: F, t4542: F, t5398: F, t5920: F, t6687: F, t6691: F, t6816: F, t7600: F, t83281: F, t88145: F, t884: F, t99209: F, t99296: F) -> F {
    let t99330 = t23384 * t28510;
    let t99336 = t28565 * t381;
    let t99353 = F::new(2.0) * t23369 * t5920 - F::cast_from(0.6092348395734171987e-3_f64) * t83281 - F::new(2.0) * t17588 * t6816 - F::new(6.0) * t3169 * t28697 - F::new(2.0) * t88145 * t1635 + F::new(4.0) * t14552 * t7600 - F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t28470 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t25784 + F::cast_from(0.18277045187202515961e-2_f64) * t99330 + F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t23329 * t99296 * t884 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t99336 * t6691 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t23329 * t23330 * t5398 * t1065 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t25423 * t17635 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t23329 * t99209 * t884;
    t99353
}
