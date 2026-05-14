//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 581/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk581<F: Float>(t1693: F, t262: F, t390: F, t1016: F, t664: F) -> (F, F, F) {
    let t2834 = t262 * t1693 * t390;
    let t2835 = 0.23744444444444444444e-1 * t2834;
    let t2836 = t664 * t1016;
    (t2834, t2835, t2836)
}
