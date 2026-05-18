//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 669/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk669<F: Float>(t14083: F, t4765: F, t49: F, t68407: F, t14030: F, t14121: F, t14123: F) -> (F, F, F, F) {
    let t68417 = t4765 * t14083 * t49;
    let t68418 = t68417 * t68407;
    let t68420 = t14030 * t14121;
    let t68421 = t68420 * t14123;
    (t68417, t68418, t68420, t68421)
}
