//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 362/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk362<F: Float>(t1099: F, t1119: F, t1086: F, t1092: F) -> (F, F) {
    let t1121 = 1.0 * t1099 * t1119;
    let t1122 = 0.17123333333333333333e-1 * t1086;
    let t1124 = -t1122 + 0.17123333333333333333e-1 * t1092;
    (t1121, t1124)
}
