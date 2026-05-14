//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 433/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk433<F: Float>(t1101: F, t1289: F, t926: F, t1507: F, t1521: F, t1547: F, t1549: F, t1553: F) -> (F, F, F) {
    let t1557 = t1101 * t1289;
    let t1558 = t926 * t1557;
    let t1561 = -t1507 + t1521 + t1547 + t1549 - t1553;
    (t1557, t1558, t1561)
}
