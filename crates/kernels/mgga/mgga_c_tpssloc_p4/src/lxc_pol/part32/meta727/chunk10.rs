//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2363/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2363<F: Float>(t104727: F, t105005: F, t105024: F, t105045: F, t105062: F, t105073: F, t105092: F, t105099: F, t112: F, t29865: F, t2169: F, t671: F) -> (F, F, F) {
    let t105102 = t104727 + t105005 + t105024 + t105045 + t105062 + t105073 + t105092 + t105099;
    let t105105 = t29865 * t112;
    let t105108 = t2169 * t671;
    (t105102, t105105, t105108)
}
