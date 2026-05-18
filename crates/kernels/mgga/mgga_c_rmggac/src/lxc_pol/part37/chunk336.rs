//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 336/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk336<F: Float>(t3191: F, t650: F, t2211: F, t664: F, t118: F, t3088: F, t3095: F, t3097: F, t1986: F, t700: F) -> (F, F, F, F, F, F, F) {
    let t3192 = t3191 * t650;
    let t3193 = F::new(0.34093327067806677161e-2) * t3192;
    let t3194 = t2211 * t664;
    let t3195 = t118 * t3194;
    let t3196 = F::new(0.39914139006212695214e-1) * t3195;
    let t3197 = F::new(0.49892673757765869017e-2) * t3088;
    let t3199 = F::new(0.10354269702074620472e-2) * t3095;
    let t3200 = F::new(0.16595192631325726674e-3) * t3097;
    let t3219 = t1986 * t700;
    (t3193, t3194, t3196, t3197, t3199, t3200, t3219)
}
