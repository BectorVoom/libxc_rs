//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 849/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk849<F: Float>(t2046: F, t3047: F, t8858: F, t8862: F, t15220: F, t2186: F, t2051: F, t577: F, t68417: F, t68406: F, t15166: F, t36639: F) -> (F, F, F, F, F, F) {
    let t75238 = t2046 * t3047 * t8858;
    let t75241 = t2046 * t3047 * t8862;
    let t75247 = t2186 * t15220;
    let t75248 = F::cast_from(0.19863479950205658386e-4_f64) * t75247;
    let t75249 = t577 * t2051;
    let t75250 = t68417 * t75249;
    let t75252 = t68406 * t75249;
    let t75254 = t36639 * t15166;
    (t75238, t75241, t75248, t75250, t75252, t75254)
}
