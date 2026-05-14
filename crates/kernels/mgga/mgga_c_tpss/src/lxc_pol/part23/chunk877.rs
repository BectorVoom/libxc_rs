//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 877/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk877<F: Float>(t3: F, t6547: F, t1670: F, t1904: F, t548: F, t6289: F, t6292: F, t6295: F, t1777: F, t3205: F) -> (F, F, F, F) {
    let t6548 = t3 * t6547;
    let t6552 = param_d * t6547;
    let t6556 = 3.0 * t1670 * t1904 + t548 * t6552 + t6289 + t6292 + t6295;
    let t7029 = t3205 * t1777;
    (t6548, t6552, t6556, t7029)
}
