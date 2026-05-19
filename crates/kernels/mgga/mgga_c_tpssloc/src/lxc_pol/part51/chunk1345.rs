//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1345/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1345<F: Float>(t32762: F, t6883: F, t1985: F, t214: F, t225: F, t26328: F, t567: F, t7722: F, t6907: F, t32761: F, t6897: F, t794: F) -> (F, F, F, F, F) {
    let t120532 = t6883 * t32762;
    let t120533 = F::cast_from(0.38381794893125283518e-1_f64) * t120532;
    let t120542 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t214 * t26328 * t225 * t567;
    let t120544 = t214 * t7722;
    let t120547 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t120544 * t6907;
    let t120550 = t6897 * t794 * t32761;
    (t120533, t120542, t120544, t120547, t120550)
}
