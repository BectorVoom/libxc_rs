//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 939/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk939<F: Float>(t23237: F, t8335: F, t1880: F, t6547: F, t8357: F, t1902: F, t234: F, t776: F, t6637: F, t6552: F, t794: F, t8356: F, t6562: F, t828: F, t232: F, t6646: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30671 = t23237 * t8335;
    let t30673 = 0.16449340668482264365e-1 * t1880 * t30671;
    let t30675 = 0.38381794893125283518e-1 * t6547 * t8357;
    let t30676 = t234 * t1902;
    let t30677 = t30676 * t776;
    let t30678 = t6637 * t30677;
    let t30680 = 0.3289868133696452873e-1 * t6552 * t30678;
    let t30681 = t794 * t8356;
    let t30683 = 0.82246703342411321825e-2 * t6562 * t30681;
    let t30684 = t1902 * t828;
    let t30685 = t30684 * t232;
    let t30686 = t6646 * t30685;
    (t30671, t30673, t30675, t30676, t30677, t30678, t30680, t30681, t30683, t30685, t30686)
}
