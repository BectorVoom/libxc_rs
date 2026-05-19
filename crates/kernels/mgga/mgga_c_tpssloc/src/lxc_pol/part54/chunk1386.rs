//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1386/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1386<F: Float>(t113005: F, t114670: F, t114673: F, t114680: F, t114689: F, t114691: F, t114694: F, t118739: F, t118743: F, t118745: F, t118751: F, t121533: F, t121536: F, t121541: F, t121546: F, t121550: F) -> F {
    let t121552 = F::cast_from(0.38381794893125283518e-1_f64) * t121533 - t118739 + t118743 - F::cast_from(0.19190897446562641759e-1_f64) * t114670 + t114673 + F::cast_from(0.19190897446562641759e-1_f64) * t121536 + t118745 + F::cast_from(0.41123351671205660912e-2_f64) * t114680 - t113005 - t114689 - F::cast_from(0.41123351671205660912e-2_f64) * t114691 + t114694 + F::cast_from(0.16449340668482264365e-1_f64) * t121541 - F::cast_from(0.82246703342411321825e-2_f64) * t121546 - t118751 + F::cast_from(0.82246703342411321825e-2_f64) * t121550;
    t121552
}
