//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2534/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2534<F: Float>(t136: F, t3297: F, t71138: F, t21746: F, t699: F, t21750: F, t50827: F, t50834: F, t63291: F, t63306: F, t63308: F, t63841: F, t63843: F, t63845: F) -> (F, F, F, F) {
    let t71333 = t136 * t3297 * t71138;
    let t71335 = t699 * t21746;
    let t71337 = t699 * t21750;
    let t71343 = -F::cast_from(0.60385000000000000002e0_f64) * t63291 + F::cast_from(0.20128333333333333334e0_f64) * t63306 - F::cast_from(0.33547222222222222222e0_f64) * t63308 - F::cast_from(0.27595e-1_f64) * t71333 + F::cast_from(0.5519e-1_f64) * t71335 - F::cast_from(0.33114e0_f64) * t71337 + t50827 - F::cast_from(0.93932222222222222225e0_f64) * t50834 - F::cast_from(0.73586666666666666666e-1_f64) * t63841 - F::cast_from(0.33114e0_f64) * t63843 + F::cast_from(0.5519e-1_f64) * t63845;
    (t71333, t71335, t71337, t71343)
}
