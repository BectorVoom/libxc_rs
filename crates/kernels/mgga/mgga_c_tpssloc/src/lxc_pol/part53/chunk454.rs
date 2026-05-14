//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 454/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk454<F: Float>(t1527: F, t865: F, t2718: F, t2627: F, t68: F, t226: F, t1509: F, t252: F, t4182: F, t1510: F, t2732: F, t4234: F, t860: F, t814: F, t829: F, t1519: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4272 = t1527 * t865;
    let t4273 = t2718 * t4272;
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4282 = t252 * t1509;
    let t4283 = t4282 * t4182;
    let t4286 = t2732 * t1510;
    let t4288 = t860 * t4234;
    let t4290 = t68 * t814;
    let t4291 = t226 * t4290;
    let t4292 = t4282 * t829;
    let t4295 = t814 * t1519;
    (t4272, t4273, t4281, t4282, t4283, t4286, t4288, t4291, t4292, t4295)
}
