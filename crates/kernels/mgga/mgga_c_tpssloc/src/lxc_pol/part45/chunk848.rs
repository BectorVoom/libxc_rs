//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 848/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk848<F: Float>(t30638: F, t6562: F, t225: F, t258: F, t6624: F, t214: F, t1880: F, t6547: F, t8332: F, t6571: F, t6662: F, t6553: F) -> (F, F, F, F, F, F, F) {
    let t30640 = F::new(0.82246703342411321825e-2) * t6562 * t30638;
    let t30642 = t6624 * t225 * t258;
    let t30643 = t214 * t30642;
    let t30645 = F::new(0.16449340668482264365e-1) * t1880 * t30643;
    let t30655 = F::new(0.38381794893125283518e-1) * t6547 * t8332;
    let t30656 = t6571 * t6662;
    let t30657 = t6553 * t30656;
    (t30640, t30642, t30643, t30645, t30655, t30656, t30657)
}
