//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 596/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk596<F: Float>(t2723: F, t2724: F, t947: F, t242: F, t2713: F, t2720: F, t941: F) -> (F, F, F) {
    let t2725 = t2723 * t2724;
    let t2726 = t947 * t2725;
    let t2727 = t242 * t2726;
    let t2731 = t2713 * t941 * t2720;
    (t2725, t2727, t2731)
}
