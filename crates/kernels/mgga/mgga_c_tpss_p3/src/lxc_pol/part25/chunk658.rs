//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 658/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk658<F: Float>(t1025: F, t4079: F, t1509: F, t2885: F, t1027: F, t1032: F, t1515: F, t673: F) -> (F, F, F, F, F) {
    let t4080 = t1025 * t4079;
    let t4087 = t2885 * t1509;
    let t4088 = t4087 * t1027;
    let t4090 = t1032 * t4079;
    let t4093 = t673 * t1515;
    (t4080, t4087, t4088, t4090, t4093)
}
