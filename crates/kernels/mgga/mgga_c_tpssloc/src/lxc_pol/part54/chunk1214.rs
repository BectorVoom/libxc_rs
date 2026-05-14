//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1214/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1214<F: Float>(t25341: F, t31366: F, t6552: F, t1880: F, t26679: F, t6553: F, t6571: F, t114592: F, t118476: F, t118479: F, t118481: F, t118484: F, t121296: F, t121299: F, t121302: F, t121305: F, t121308: F, t121311: F) -> (F,) {
    let t121314 = t6552 * t31366 * t25341;
    let t121318 = t1880 * t6553 * t6571 * t26679;
    let t121320 = t118476 + t118479 - 0.82246703342411321824e-2 * t114592 - t118481 + 0.19190897446562641759e-1 * t121296 + 0.16449340668482264365e-1 * t121299 + t118484 - 0.82246703342411321825e-2 * t121302 + 0.41123351671205660912e-2 * t121305 - 0.82246703342411321825e-2 * t121308 - 0.16449340668482264365e-1 * t121311 - 0.16449340668482264365e-1 * t121314 - 0.82246703342411321825e-2 * t121318;
    (t121320,)
}
