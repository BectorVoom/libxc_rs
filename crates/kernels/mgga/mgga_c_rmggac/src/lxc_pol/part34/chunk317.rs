//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 317/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk317<F: Float>(t649: F, t664: F, t27: F, t640: F, t702: F, t3066: F, t3070: F, t3082: F, t36: F, t699: F, t305: F, t326: F, t650: F, t2211: F, t118: F, t3088: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3166 = t649 * t664;
    let t3167 = t27 * t3166;
    let t3180 = t640 * t702;
    let t3184 = 0.14967802127329760705e-1 * t3066;
    let t3185 = 0.10227998120342003148e-1 * t3070;
    let t3187 = 0.68186654135613354325e-2 * t3082;
    let t3188 = t699 * t36;
    let t3189 = t305 * t3188;
    let t3190 = 0.14967802127329760705e-1 * t3189;
    let t3191 = t326 * t699;
    let t3192 = t3191 * t650;
    let t3193 = 0.34093327067806677161e-2 * t3192;
    let t3194 = t2211 * t664;
    let t3195 = t118 * t3194;
    let t3196 = 0.39914139006212695214e-1 * t3195;
    let t3197 = 0.49892673757765869017e-2 * t3088;
    (t3167, t3180, t3184, t3185, t3187, t3188, t3190, t3191, t3193, t3194, t3196, t3197)
}
