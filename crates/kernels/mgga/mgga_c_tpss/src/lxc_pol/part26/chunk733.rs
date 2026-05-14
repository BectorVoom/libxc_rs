//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 733/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk733<F: Float>(t124: F, t4706: F, t762: F, t4701: F, t1378: F) -> (F, F, F, F, F) {
    let t4707 = t124 * t4706;
    let t4708 = t762 * t4707;
    let t4711 = t124 * t4701;
    let t4712 = t762 * t4711;
    let t4715 = t1378 * t1378;
    (t4707, t4708, t4711, t4712, t4715)
}
