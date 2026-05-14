//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 370/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk370<F: Float>(t1184: F, t457: F, t460: F, t974: F, t1173: F, t1174: F, t1180: F) -> (F, F, F) {
    let t1185 = t457 * t1184;
    let t1186 = t1185 * t460;
    let t1187 = t974 * t1186;
    let t1190 = t1173 - 0.27777777777777777777e-3 * t1174 * t1180 - 0.83333333333333333332e-3 * t1174 * t1187;
    (t1186, t1187, t1190)
}
