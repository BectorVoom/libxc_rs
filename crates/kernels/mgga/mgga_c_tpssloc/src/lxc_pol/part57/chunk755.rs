//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 755/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk755<F: Float>(t226: F, t30719: F, t8344: F, t6547: F, t8336: F, t2015: F, t3886: F, t1377: F, t794: F, t8454: F, t6897: F, t6883: F, t8455: F, t8459: F, t22674: F, t8458: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30748 = 0.38381794893125283518e-1 * t6547 * t8336;
    let t31090 = t3886 * t2015;
    let t31099 = t1377 * t2015;
    let t31104 = t794 * t8454;
    let t31106 = 0.82246703342411321825e-2 * t6897 * t31104;
    let t31113 = 0.38381794893125283518e-1 * t6883 * t8455;
    let t31115 = 0.38381794893125283518e-1 * t6883 * t8459;
    let t31127 = t22674 * t8458;
    (t30720, t30721, t30748, t31090, t31099, t31104, t31106, t31113, t31115, t31127)
}
