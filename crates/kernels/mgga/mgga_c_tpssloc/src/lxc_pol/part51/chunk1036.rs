//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1036/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1036<F: Float>(t30656: F, t6553: F, t1880: F, t23204: F, t8335: F, t6562: F, t1902: F, t214: F) -> (F, F, F, F, F) {
    let t30657 = t6553 * t30656;
    let t30659 = 0.16449340668482264365e-1 * t1880 * t30657;
    let t30660 = t23204 * t8335;
    let t30662 = 0.82246703342411321825e-2 * t6562 * t30660;
    let t30663 = t214 * t1902;
    (t30657, t30659, t30660, t30662, t30663)
}
