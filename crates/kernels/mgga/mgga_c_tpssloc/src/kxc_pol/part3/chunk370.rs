//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 370/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk370<F: Float>(t1178: F, t607: F, t1177: F, t1111: F, t1115: F) -> (F, F, F) {
    let t1179 = t1178 * t607;
    let t1180 = t1177 * t1179;
    let t1184 = t1111 / F::cast_from(6.0_f64) - t1115 / F::cast_from(6.0_f64);
    (t1179, t1180, t1184)
}
