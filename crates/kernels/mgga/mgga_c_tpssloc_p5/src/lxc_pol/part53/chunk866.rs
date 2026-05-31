//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 866/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk866<F: Float>(t31236: F, t5113: F, t8326: F, t3938: F, t671: F, t3941: F, t191: F, t192: F, t7166: F, t225: F, t258: F, t7084: F) -> (F, F, F, F, F, F, F) {
    let t31237 = F::cast_from(2.0_f64) * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = F::cast_from(2.0_f64) * t31238;
    let t31283 = t3938 * t8326;
    let t31284 = F::cast_from(0.135e2_f64) * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = F::cast_from(27.0_f64) * t31286;
    let t31304 = t7166 * t191 * t192;
    let t31315 = t7084 * t225 * t258;
    (t31237, t31239, t31284, t31285, t31287, t31304, t31315)
}
