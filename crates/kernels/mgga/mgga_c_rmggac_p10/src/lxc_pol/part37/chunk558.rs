//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 558/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk558<F: Float>(t13859: F, t13869: F, t13873: F, t13877: F, t13881: F, t13885: F, t14438: F, t739: F, t2106: F, t3224: F, t2145: F, t2160: F, t3180: F, t638: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14536 = F::cast_from(0.85129199786595678799e-5_f64) * t13859;
    let t14538 = F::cast_from(0.15961724959986689775e-4_f64) * t13869;
    let t14539 = F::cast_from(0.1276937996798935182e-4_f64) * t13873;
    let t14540 = F::cast_from(0.2553875993597870364e-4_f64) * t13877;
    let t14541 = F::cast_from(0.3830813990396805546e-4_f64) * t13881;
    let t14542 = F::cast_from(0.1276937996798935182e-4_f64) * t13885;
    let t14549 = t739 * t14438;
    let t14550 = F::cast_from(0.14967802127329760705e-1_f64) * t14549;
    let t14551 = t3224 * t2106;
    let t14552 = t2145 * t14551;
    let t14557 = t638 * t2160 * t3180;
    (t14536, t14538, t14539, t14540, t14541, t14542, t14550, t14551, t14552, t14557)
}
