//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 252/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk252<F: Float>(t252: F, t798: F, t154: F, t782: F, t222: F, t119: F, t776: F, t210: F) -> (F, F, F, F, F) {
    let t799 = t798 * t252;
    let t801 = t782 * t154;
    let t803 = F::new(7.0) / F::new(288.0) * t801 * t222;
    let t804 = t119 * t776;
    let t805 = t210 * t804;
    (t799, t801, t803, t804, t805)
}
