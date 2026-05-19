//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 336/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk336<F: Float>(t3191: F, t650: F, t2211: F, t664: F, t118: F, t3088: F, t3095: F, t3097: F, t3091: F, t3100: F, t3103: F) -> (F, F, F, F) {
    let t3192 = t3191 * t650;
    let t3193 = F::cast_from(0.34093327067806677161e-2_f64) * t3192;
    let t3194 = t2211 * t664;
    let t3195 = t118 * t3194;
    let t3196 = F::cast_from(0.39914139006212695214e-1_f64) * t3195;
    let t3197 = F::cast_from(0.49892673757765869017e-2_f64) * t3088;
    let t3199 = F::cast_from(0.10354269702074620472e-2_f64) * t3095;
    let t3200 = F::cast_from(0.16595192631325726674e-3_f64) * t3097;
    let t3203 = t3197 - F::cast_from(0.34093327067806677161e-2_f64) * t3091 + t3199 + t3200 - F::cast_from(0.90720386384580639149e-4_f64) * t3100 + F::cast_from(0.24108102678124669848e-4_f64) * t3103;
    (t3193, t3194, t3196, t3203)
}
