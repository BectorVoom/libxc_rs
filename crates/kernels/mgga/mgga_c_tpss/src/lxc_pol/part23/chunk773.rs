//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 773/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk773<F: Float>(t1139: F, t4322: F, t1136: F, t1149: F, t1587: F, t3113: F, t4294: F, t4296: F, t4300: F, t473: F) -> (F, F) {
    let t4323 = t1139 * t4322;
    let t4325 = 2.0 * t1136 * t4300 - t1136 * t4323 - t1149 * t4296 - t1587 * t3113 + t4294 * t473;
    (t4323, t4325)
}
