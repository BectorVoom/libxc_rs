//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1343/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1343<F: Float>(t212: F, t23171: F, t6554: F, t852: F, t22986: F, t23270: F, t2717: F, t2719: F, t776: F, t23030: F, t23253: F, t23204: F, t23241: F, t81640: F) -> (F, F, F, F) {
    let t82087 = t23171 * t212 * t852 * t6554;
    let t82092 = t22986 * t23270 * t2717 * t2719 * t776;
    let t82099 = t23030 * t23253;
    let t82108 = t81640 * t23204 * t23241;
    (t82087, t82092, t82099, t82108)
}
