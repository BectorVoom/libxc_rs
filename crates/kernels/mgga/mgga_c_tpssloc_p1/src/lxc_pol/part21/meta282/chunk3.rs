//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1570/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1570<F: Float>(t10224: F, t979: F, t973: F, t2970: F, t2999: F, t135: F, t2978: F) -> (F, F, F, F) {
    let t10225 = t10224 * t979;
    let t10226 = t973 * t10225;
    let t10228 = t2970 * t2999;
    let t10229 = t973 * t10228;
    let t10231 = t135 * t2978;
    (t10225, t10226, t10229, t10231)
}
