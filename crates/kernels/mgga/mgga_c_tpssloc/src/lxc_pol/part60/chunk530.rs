//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 530/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk530<F: Float>(t1170: F, t2148: F, t2121: F, t225: F, t7284: F, t477: F, t491: F, t50: F, t6794: F, t131: F, t467: F, t1009: F, t461: F, t1209: F, t475: F, t68: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7359 = t1170 * t2148;
    let t7361 = 0.27415567780803773942e-2 * t2121 * t7359;
    let t7362 = t7284 * t225;
    let t7363 = t477 * t491;
    let t7371 = t50 * t6794;
    let t7372 = t7371 * t131;
    let t7373 = t7372 * t467;
    let t7374 = t461 * t1009;
    let t7375 = t7374 * t1209;
    let t7376 = t68 * t475;
    (t7359, t7361, t7362, t7363, t7371, t7372, t7373, t7375, t7376)
}
