//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1301/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1301<F: Float>(t606: F, t7540: F, t1408: F, t6665: F, t1530: F, t25373: F, t776: F, t22960: F, t1484: F, t7537: F, t857: F, t22986: F, t23270: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118393 = t606 * t7540;
    let t118410 = t1408 * t6665;
    let t118413 = t1530 * t6665;
    let t118414 = t25373 * t118413;
    let t118454 = t7540 * t776;
    let t118455 = t22960 * t118454;
    let t118466 = t1484 * t6665;
    let t118467 = t22960 * t118466;
    let t118472 = t857 * t7537;
    let t118476 = F::new(0.3289868133696452873e-1) * t22986 * t23270 * t118472 * t776;
    (t118393, t118410, t118413, t118414, t118454, t118455, t118466, t118467, t118476)
}
