//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1241/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1241<F: Float>(t10469: F, t10474: F, t363: F, t10401: F, t23417: F, t3186: F, t10383: F, t1926: F, t10472: F, t10478: F, t23535: F, t6753: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t82989 = t10469 * t10474 * t363;
    let t83015 = t23417 * t10401;
    let t83016 = t3186 * t83015;
    let t83028 = F::new(5.0) / F::new(1296.0) * t1926 * t10383;
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    let t83065 = t10472 * t6753 * t10478;
    (t82989, t83015, t83016, t83028, t83054, t83058, t83065)
}
