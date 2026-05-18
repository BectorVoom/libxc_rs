//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 184/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk184<F: Float>(t118: F, t794: F, t207: F, t792: F, t154: F, t782: F, t222: F, t226: F, t68: F) -> (F, F, F, F, F) {
    let t795 = t118 * t794;
    let t797 = F::new(0.41666666666666666666e-3) * t792 * t207 * t795;
    let t801 = t782 * t154;
    let t803 = F::new(7.0) / F::new(288.0) * t801 * t222;
    let t812 = t226 * t68;
    (t795, t797, t801, t803, t812)
}
