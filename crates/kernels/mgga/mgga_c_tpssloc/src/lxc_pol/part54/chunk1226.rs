//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1226/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1226<F: Float>(t23168: F, t33376: F, t33380: F, t6579: F, t1888: F, t22996: F, t26657: F, t232: F, t6646: F, t7823: F, t828: F, t1880: F, t1894: F, t214: F, t26653: F, t113005: F, t114670: F, t114673: F, t114680: F, t114689: F, t114691: F, t114694: F, t118739: F, t118743: F, t118745: F, t118751: F) -> (F,) {
    let t121533 = t23168 * t33376;
    let t121536 = t6579 * t33380;
    let t121541 = t1888 * t22996 * t26657;
    let t121546 = t1888 * t6646 * t7823 * t828 * t232;
    let t121550 = t1880 * t214 * t1894 * t26653;
    let t121552 = 0.38381794893125283518e-1 * t121533 - t118739 + t118743 - 0.19190897446562641759e-1 * t114670 + t114673 + 0.19190897446562641759e-1 * t121536 + t118745 + 0.41123351671205660912e-2 * t114680 - t113005 - t114689 - 0.41123351671205660912e-2 * t114691 + t114694 + 0.16449340668482264365e-1 * t121541 - 0.82246703342411321825e-2 * t121546 - t118751 + 0.82246703342411321825e-2 * t121550;
    (t121552,)
}
