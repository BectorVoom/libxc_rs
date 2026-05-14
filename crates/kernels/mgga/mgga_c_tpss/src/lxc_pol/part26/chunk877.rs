//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 877/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk877<F: Float>(t6513: F, t935: F, t1586: F, t1880: F, t6025: F, t1561: F, t342: F, t450: F) -> (F, F, F, F) {
    let t6514 = t6513 * t935;
    let t6516 = t1880 * t1586;
    let t6517 = t6025 * t6516;
    let t6521 = t1561 * t342 * t450;
    (t6514, t6516, t6517, t6521)
}
