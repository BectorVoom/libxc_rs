//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 268/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk268<F: Float>(t1176: F, t974: F, t1089: F, t461: F, t607: F, t1111: F, t1115: F) -> (F, F, F, F, F) {
    let t1177 = t974 * t1176;
    let t1178 = t461 * t1089;
    let t1179 = t1178 * t607;
    let t1180 = t1177 * t1179;
    let t1184 = t1111 / 6.0 - t1115 / 6.0;
    (t1177, t1178, t1179, t1180, t1184)
}
