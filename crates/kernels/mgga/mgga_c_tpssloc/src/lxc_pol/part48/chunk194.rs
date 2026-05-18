//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 194/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk194<F: Float>(t207: F, t215: F, t782: F, t154: F, t229: F, t205: F, t210: F, t214: F, t776: F, t16: F, t59: F, t120: F, t212: F) -> (F, F, F, F, F, F) {
    let t785 = F::new(0.19444444444444444444e-2) * t782 * t207 * t215;
    let t786 = t154 * t229;
    let t787 = t205 * t786;
    let t789 = t210 * t214 * t776;
    let t792 = t59 * t16;
    let t794 = t120 * t212;
    (t785, t786, t787, t789, t792, t794)
}
