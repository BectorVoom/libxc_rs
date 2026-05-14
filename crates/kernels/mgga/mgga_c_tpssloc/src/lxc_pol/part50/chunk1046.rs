//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1046/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1046<F: Float>(t193: F, t8418: F, t200: F, t8369: F, t22960: F, t4255: F, t7540: F, t776: F, t1877: F, t2219: F, t8366: F, t1484: F, t6665: F, t7537: F, t857: F, t22986: F, t23270: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118436 = t193 * t8418;
    let t118439 = t193 * t200 * t8369;
    let t118440 = t22960 * t4255;
    let t118454 = t7540 * t776;
    let t118455 = t22960 * t118454;
    let t118465 = t1877 * t8366 * t2219;
    let t118466 = t1484 * t6665;
    let t118467 = t22960 * t118466;
    let t118472 = t857 * t7537;
    let t118476 = 0.3289868133696452873e-1 * t22986 * t23270 * t118472 * t776;
    (t118436, t118439, t118440, t118454, t118455, t118465, t118466, t118467, t118476)
}
