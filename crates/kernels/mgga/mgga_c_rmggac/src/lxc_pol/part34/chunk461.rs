//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 461/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk461<F: Float>(t352: F, t9540: F, t118: F, t305: F, t326: F, t4669: F, t5259: F, t5266: F, t7793: F, t7796: F, t7816: F, t8919: F, t8921: F, t8926: F, t9332: F, t9340: F, t9352: F, t9370: F, t9524: F, t9527: F, t9531: F) -> (F, F) {
    let t9541 = t9540 * t352;
    let t9546 = -F::cast_from(0.20455996240684006298e-1_f64) * t8919 + F::cast_from(0.2727466165424534173e-1_f64) * t8921 + F::cast_from(0.68186654135613354325e-2_f64) * t8926 + F::cast_from(0.79828278012425390427e-1_f64) * t7793 - F::cast_from(0.17961362552795712846e0_f64) * t4669 * t9524 + F::cast_from(0.11974241701863808564e0_f64) * t5259 * t9527 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t9531 + t7796 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t9352 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t9340 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t9370 + F::cast_from(0.11974241701863808564e0_f64) * t5266 * t9541 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t9332 - t7816;
    (t9541, t9546)
}
