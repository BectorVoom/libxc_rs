//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 573/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk573<F: Float>(t466: F, t7348: F, t2145: F, t225: F, t1251: F, t2154: F, t3598: F, t1170: F, t2148: F, t2121: F, t7284: F, t477: F, t491: F, t1090: F, t1186: F, t50: F, t6794: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7349 = t466 * t7348;
    let t7351 = t2145 * t225;
    let t7355 = t2154 * t1251;
    let t7356 = t3598 * t7355;
    let t7359 = t1170 * t2148;
    let t7361 = 0.27415567780803773942e-2 * t2121 * t7359;
    let t7362 = t7284 * t225;
    let t7363 = t477 * t491;
    let t7364 = t7363 * t1090;
    let t7365 = t7362 * t7364;
    let t7368 = t1186 * t2148;
    let t7371 = t50 * t6794;
    (t7349, t7351, t7356, t7361, t7362, t7363, t7364, t7365, t7368, t7371)
}
