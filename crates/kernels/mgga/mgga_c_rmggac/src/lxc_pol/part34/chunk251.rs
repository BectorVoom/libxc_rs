//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 251/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk251<F: Float>(t2106: F, t655: F, t2069: F, t851: F, t2074: F, t854: F, t265: F, t344: F, t22: F) -> (F, F, F, F, F, F, F) {
    let t2107 = t655 * t2106;
    let t2108 = 0.30305179615828157477e-2 * t2107;
    let t2109 = t851 * t2069;
    let t2111 = t854 * t2074;
    let t2113 = t344 * t265;
    let t2114 = 0.17701538806747441785e-3 * t2113;
    let t2115 = t854 * t22;
    (t2107, t2108, t2109, t2111, t2113, t2114, t2115)
}
