//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 517/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk517<F: Float>(t118: F, t1691: F, t1780: F, t1865: F, t1897: F, t1899: F, t485: F, t544: F, t3: F) -> (F, F, F) {
    let t1901 = -t118 * t1897 - t1865 * t485 + t1899 * t544 - t1691 + t1780;
    let t1902 = t3 * t1901;
    let t1904 = param_d * t1901;
    (t1901, t1902, t1904)
}
