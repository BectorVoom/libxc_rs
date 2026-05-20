//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1471/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1471<F: Float>(t10655: F, t2845: F, t10521: F, t10528: F, t10607: F, t10622: F, t10625: F, t10627: F, t10635: F, t10649: F, t10652: F, t10654: F) -> (F, F) {
    let t10657 = F::cast_from(0.48245938496077605201e2_f64) * t10655 * t2845;
    let t10658 = -t10521 + t10528 - t10607 + t10622 - t10625 - t10627 - t10635 - t10649 + t10652 + t10654 + t10657;
    (t10657, t10658)
}
