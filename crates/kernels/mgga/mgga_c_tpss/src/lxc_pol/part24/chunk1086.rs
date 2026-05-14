//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1086/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1086<F: Float>(t1038: F, t15275: F, t141: F, t15271: F, t15286: F, t15266: F, t2895: F, t15262: F, t15257: F, t9185: F, t15281: F, t11938: F, t12129: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F) -> (F, F, F, F, F, F, F, F) {
    let t15320 = t1038 * t15275;
    let t15321 = t141 * t15320;
    let t15323 = t1038 * t15271;
    let t15324 = t141 * t15323;
    let t15326 = t1038 * t15286;
    let t15327 = t141 * t15326;
    let t15329 = t2895 * t15266;
    let t15330 = t141 * t15329;
    let t15333 = t2895 * t15262;
    let t15334 = t141 * t15333;
    let t15338 = t9185 * t15257;
    let t15339 = t141 * t15338;
    let t15341 = t2895 * t15281;
    let t15342 = t141 * t15341;
    let t15349 = -0.16557e0 * t15334 + 0.26837777777777777779e0 * t11938 - t12129 - 0.20128333333333333333e0 * t15283 + 0.36793333333333333333e-1 * t15339 - 0.27595e-1 * t15342 - 0.40256666666666666666e0 * t15268 - 0.12077e1 * t15264 + 0.12077e1 * t15277 + 0.181155e1 * t15273 + 0.60385e0 * t15288;
    (t15321, t15324, t15327, t15330, t15334, t15339, t15342, t15349)
}
