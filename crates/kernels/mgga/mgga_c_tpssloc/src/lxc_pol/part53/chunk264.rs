//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 264/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk264<F: Float>(t1176: F, t974: F, t1089: F, t461: F, t607: F, t1111: F, t1115: F, t457: F, t460: F, t1173: F, t1174: F, t491: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1177 = t974 * t1176;
    let t1178 = t461 * t1089;
    let t1179 = t1178 * t607;
    let t1180 = t1177 * t1179;
    let t1184 = t1111 / F::new(6.0) - t1115 / F::new(6.0);
    let t1185 = t457 * t1184;
    let t1186 = t1185 * t460;
    let t1187 = t974 * t1186;
    let t1190 = t1173 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t1180 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t1187;
    let t1191 = t1190 * t491;
    (t1177, t1178, t1179, t1180, t1184, t1186, t1187, t1190, t1191)
}
