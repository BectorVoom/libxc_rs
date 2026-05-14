//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 854/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk854<F: Float>(t3: F, t6061: F, t1281: F, t1904: F, t548: F, t5771: F, t5775: F, t5778: F, t3418: F, t38: F) -> (F, F, F, F) {
    let t6062 = t3 * t6061;
    let t6067 = param_d * t6061;
    let t6071 = 3.0 * t1281 * t1904 + t548 * t6067 + t5771 + t5775 + t5778;
    let t6073 = t3418 * t38;
    (t6062, t6067, t6071, t6073)
}
