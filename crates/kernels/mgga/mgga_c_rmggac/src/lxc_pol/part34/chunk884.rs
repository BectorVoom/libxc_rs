//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 884/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk884<F: Float>(t75352: F, t75360: F, t75375: F, t69294: F, t75386: F, t75388: F, t75390: F, t75356: F, t75362: F, t75364: F, t75367: F, t75369: F, t75371: F, t75378: F, t75380: F, t75383: F) -> (F,) {
    let t78148 = 0.31752135234603223702e-2 * t75352;
    let t78150 = 0.72324308034374009545e-3 * t75360;
    let t78156 = 0.31062809106223861416e-2 * t75375;
    let t78157 = 0.79828278012425390427e-1 * t69294;
    let t78161 = 0.62125618212447722832e-2 * t75386;
    let t78162 = 0.15531404553111930708e-1 * t75388;
    let t78163 = 0.15531404553111930708e-1 * t75390;
    let t78164 = t78148 - 0.50803416375365157923e-2 * t75356 + t78150 + 0.24192103035888170439e-2 * t75362 - 0.33868944250243438615e-2 * t75364 - 0.68186654135613354322e-2 * t75367 - 0.68186654135613354322e-2 * t75369 + 0.13637330827122670864e-1 * t75371 + t78156 + t78157 + 0.27274661654245341729e-1 * t75378 + 0.27274661654245341728e-1 * t75380 - 0.6818665413561335432e-1 * t75383 + t78161 - t78162 - t78163;
    (t78164,)
}
