//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2012/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2012<F: Float>(t3186: F, t83015: F, t3158: F, t6712: F, t10383: F, t1926: F, t10948: F, t23536: F, t10472: F, t10474: F, t10478: F, t23535: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t83016 = t3186 * t83015;
    let t83025 = t6712 * t3158;
    let t83028 = F::new(5.0) / F::new(1296.0) * t1926 * t10383;
    let t83043 = t10948 * t23536;
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    (t83016, t83025, t83028, t83043, t83054, t83058)
}
