//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 845/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk845<F: Float>(t114790: F, t23164: F, t6555: F, t2047: F, t212: F, t23171: F, t6554: F, t31420: F, t6547: F, t23228: F, t8547: F, t31370: F, t23204: F, t31419: F, t6562: F, t31650: F, t6883: F) -> (F, F, F, F, F, F, F) {
    let t114916 = t23164 * t114790 * t6555;
    let t114932 = t23171 * t212 * t2047 * t6554;
    let t114939 = t6547 * t31420;
    let t114943 = t23171 * t23228 * t8547;
    let t114945 = t6547 * t31370;
    let t114965 = t6562 * t23204 * t31419;
    let t115292 = t6883 * t31650;
    (t114916, t114932, t114939, t114943, t114945, t114965, t115292)
}
