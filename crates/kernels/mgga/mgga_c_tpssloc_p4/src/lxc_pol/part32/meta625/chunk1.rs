//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2034/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2034<F: Float>(t23164: F, t6555: F, t86893: F, t7480: F, t81632: F, t23030: F, t25035: F, t23228: F, t7479: F, t81573: F, t25059: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t86895 = t23164 * t86893 * t6555;
    let t86896 = F::cast_from(0.16449340668482264365e-1_f64) * t86895;
    let t86903 = t81632 * t7480;
    let t86911 = t23030 * t25035;
    let t86916 = t81573 * t23228 * t7479;
    let t86928 = t6562 * t794 * t25059;
    (t86896, t86903, t86911, t86916, t86928)
}
