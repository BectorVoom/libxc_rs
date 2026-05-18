//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 850/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk850<F: Float>(t1911: F, t857: F, t2717: F, t794: F, t8331: F, t6562: F, t6547: F, t8332: F, t23204: F, t8335: F, t1902: F, t214: F) -> (F, F, F, F, F, F, F, F) {
    let t30622 = t857 * t1911;
    let t30633 = t2717 * t1911;
    let t30638 = t794 * t8331;
    let t30640 = F::new(0.82246703342411321825e-2) * t6562 * t30638;
    let t30655 = F::new(0.38381794893125283518e-1) * t6547 * t8332;
    let t30660 = t23204 * t8335;
    let t30662 = F::new(0.82246703342411321825e-2) * t6562 * t30660;
    let t30663 = t214 * t1902;
    (t30622, t30633, t30638, t30640, t30655, t30660, t30662, t30663)
}
