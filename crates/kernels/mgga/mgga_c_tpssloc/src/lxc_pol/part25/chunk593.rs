//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 593/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk593<F: Float>(t2600: F, t541: F, t1329: F, t3726: F, t119: F, t3734: F, t210: F, t3719: F, t225: F, t3752: F) -> (F, F, F, F, F) {
    let t3762 = F::new(35.0) / F::new(432.0) * t2600 * t541;
    let t3763 = t3726 * t1329;
    let t3765 = t119 * t3734;
    let t3766 = t210 * t3765;
    let t3770 = t210 * t119 * t3719;
    let t3773 = t3752 * t225;
    (t3762, t3763, t3766, t3770, t3773)
}
