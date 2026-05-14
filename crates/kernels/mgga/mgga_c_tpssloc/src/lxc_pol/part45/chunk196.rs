//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 196/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk196<F: Float>(t181: F, t676: F, t686: F, t756: F, t172: F, t187: F, t739: F, t745: F, t746: F) -> (F, F, F, F) {
    let t758 = t686 * t676 * t181;
    let t760 = 0.18311447306006545054e-3 * t756 * t758;
    let t761 = t187 * t172;
    let t763 = t739 * t745 * t746;
    (t758, t760, t761, t763)
}
