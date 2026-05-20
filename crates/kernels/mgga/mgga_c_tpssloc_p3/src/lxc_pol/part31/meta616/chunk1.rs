//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1864/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1864<F: Float>(t1390: F, t19631: F, t1845: F, t5356: F, t22674: F, t28191: F, t80681: F, t1985: F, t22666: F, t28232: F, t26331: F, t26333: F, t90566: F) -> (F, F, F, F, F) {
    let t96824 = t1390 * t19631;
    let t96830 = t1845 * t5356;
    let t96848 = t80681 * t22674 * t28191;
    let t96851 = t1985 * t22666 * t28232;
    let t96854 = t26331 * t90566 * t26333;
    (t96824, t96830, t96848, t96851, t96854)
}
