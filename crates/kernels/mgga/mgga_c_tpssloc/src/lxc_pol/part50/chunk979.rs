//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 979/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk979<F: Float>(t31283: F, t671: F, t8326: F, t3941: F, t31253: F, t31267: F, t31270: F, t31272: F, t31274: F, t31277: F, t31279: F, t31282: F, t577: F, t8508: F, t1873: F, t649: F) -> (F, F, F, F, F) {
    let t31284 = 0.135e2 * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = 27.0 * t31286;
    let t31288 = 0.45e1 * t31253 * t577 + 0.135e2 * t31267 * t671 + 27.0 * t31270 + 54.0 * t31272 + 27.0 * t31274 + t31277 + t31279 + t31282 + t31284 + t31287 + t8508;
    let t31537 = t649 * t1873;
    (t31284, t31285, t31287, t31288, t31537)
}
