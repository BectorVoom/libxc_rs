//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 738/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk738<F: Float>(t1990: F, t7939: F, t2186: F, t7682: F, t7905: F, t271: F, t4765: F, t4768: F, t7325: F, t2164: F, t7323: F, t7324: F) -> (F, F, F, F, F) {
    let t34907 = t7939 * t1990;
    let t34911 = t2186 * t7682;
    let t34913 = t2186 * t7905;
    let t34921 = t4765 * t4768 * t271 * t7325;
    let t34922 = F::cast_from(0.64980365807044550255e-5_f64) * t34921;
    let t34927 = t7323 * t2164 * t7324;
    (t34907, t34911, t34913, t34922, t34927)
}
