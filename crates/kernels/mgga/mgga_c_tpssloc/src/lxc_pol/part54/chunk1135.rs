//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1135/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1135<F: Float>(t6547: F, t8332: F, t6571: F, t6662: F, t6553: F, t1880: F, t23204: F, t8335: F, t6562: F, t1902: F, t214: F) -> (F, F, F, F, F, F, F) {
    let t30655 = F::cast_from(0.38381794893125283518e-1_f64) * t6547 * t8332;
    let t30656 = t6571 * t6662;
    let t30657 = t6553 * t30656;
    let t30659 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t30657;
    let t30660 = t23204 * t8335;
    let t30662 = F::cast_from(0.82246703342411321825e-2_f64) * t6562 * t30660;
    let t30663 = t214 * t1902;
    (t30655, t30656, t30657, t30659, t30660, t30662, t30663)
}
