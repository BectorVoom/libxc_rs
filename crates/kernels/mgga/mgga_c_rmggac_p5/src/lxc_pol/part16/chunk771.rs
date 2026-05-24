//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 771/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk771<F: Float>(t1969: F, t8516: F, t7229: F, t7243: F, t2134: F, t27: F, t3118: F, t321: F, t504: F, t7262: F, t507: F, t7191: F) -> (F, F, F, F, F) {
    let t36336 = t8516 * t1969;
    let t36343 = t7229 * t7243;
    let t36402 = t2134 * t27 * t3118 * t321;
    let t36457 = t504 * t7262;
    let t36471 = t507 * t7191;
    (t36336, t36343, t36402, t36457, t36471)
}
