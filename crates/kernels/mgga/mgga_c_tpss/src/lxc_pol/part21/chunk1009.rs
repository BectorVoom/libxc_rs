//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1009/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1009<F: Float>(t11004: F, t10982: F, t1289: F, t8493: F, t1985: F, t8609: F, t128: F) -> (F, F, F, F) {
    let t11005 = 4.0 / 9.0 * t11004;
    let t11006 = 2.0 / 9.0 * t10982;
    let t11007 = t8493 * t1289;
    let t11008 = t11007 * t1985;
    let t11009 = t8609 * t11008;
    let t11010 = t128 * t11009;
    (t11005, t11006, t11008, t11010)
}
