//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 780/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk780<F: Float>(t31286: F, t191: F, t192: F, t7166: F, t225: F, t258: F, t7084: F, t214: F, t1880: F, t794: F, t8537: F, t6562: F, t23237: F, t8547: F, t2053: F, t2717: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31287 = 27.0 * t31286;
    let t31304 = t7166 * t191 * t192;
    let t31315 = t7084 * t225 * t258;
    let t31316 = t214 * t31315;
    let t31317 = t1880 * t31316;
    let t31319 = t794 * t8537;
    let t31320 = t6562 * t31319;
    let t31329 = t23237 * t8547;
    let t31330 = t1880 * t31329;
    let t31332 = t2717 * t2053;
    (t31287, t31304, t31315, t31316, t31317, t31319, t31320, t31329, t31330, t31332)
}
