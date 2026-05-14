//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1278/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1278<F: Float>(t21024: F, t5706: F, t1206: F, t21011: F, t19620: F, t7029: F, t21236: F, t5532: F, t19305: F, t6106: F, t19308: F, t19327: F, t6103: F, t1760: F, t19571: F, t4525: F) -> (F, F, F, F, F, F, F) {
    let t68837 = 2.0 * t5706 * t21024;
    let t68838 = t21011 * t1206;
    let t68841 = 12.0 * t19620 * t7029 * t68838;
    let t68843 = 2.0 * t21236 * t5532;
    let t68845 = 4.0 * t19305 * t6106;
    let t68848 = 4.0 * t19308 * t6106;
    let t68850 = 4.0 * t6103 * t19327;
    let t68853 = 2.0 * t1760 * t19571 * t4525;
    (t68837, t68841, t68843, t68845, t68848, t68850, t68853)
}
