//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1127/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1127<F: Float>(t10472: F, t10474: F, t10478: F, t23535: F, t6753: F, t10375: F, t1942: F, t1014: F, t10469: F, t363: F, t3127: F, t3200: F, t83015: F, t25511: F, t6743: F, t23592: F, t23631: F, t974: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    let t83065 = t10472 * t6753 * t10478;
    let t83080 = t1942 * t10375 / 5184.0;
    let t83142 = t10469 * t1014 * t363;
    let t83196 = t10469 * t3127 * t363;
    let t83215 = t3200 * t83015;
    let t83233 = t6743 * t25511;
    let t83239 = t23631 * t974 * t23592;
    (t83054, t83058, t83065, t83080, t83142, t83196, t83215, t83233, t83239)
}
