//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1202/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1202<F: Float>(t1920: F, t32938: F, t968: F, t362: F, t7593: F, t1014: F, t113578: F, t1539: F, t1945: F, t23327: F, t23478: F, t23601: F, t23602: F, t23657: F, t25429: F, t25486: F, t25492: F, t25496: F, t25510: F, t25712: F, t2770: F, t2775: F, t30877: F, t3127: F, t32934: F, t3961: F, t4347: F, t6687: F, t6784: F, t6797: F, t6799: F, t6800: F, t884: F) -> F {
    let t119177 = t1920 * t968 * t32938;
    let t119179 = t362 * t7593;
    let t119201 = -F::new(0.16449340668482264365e-1) * t6797 * t23657 * t32934 + F::new(0.3289868133696452873e-1) * t23601 * t23602 * t3127 * t1945 * t25486 + F::new(0.16449340668482264365e-1) * t6797 * t6799 * t25496 * t6800 - F::new(0.10966227112321509577e-1) * t23327 * t25510 * t1945 * t2775 * t3961 + F::new(0.73108180748810063844e-2) * t25429 * t25510 * t1945 * t2770 * t3961 + F::new(0.54831135561607547883e-2) * t119177 + F::new(0.54831135561607547883e-2) * t6687 * t6784 * t119179 * t884 - F::new(0.16449340668482264365e-1) * t23601 * t23602 * t1014 * t1945 * t25492 - F::new(0.16449340668482264365e-1) * t6687 * t25712 * t23478 * t30877 + F::new(0.54831135561607547883e-2) * t6687 * t6784 * t113578 * t1539 + F::new(0.54831135561607547883e-2) * t6687 * t6784 * t30877 * t4347;
    t119201
}
