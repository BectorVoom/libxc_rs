//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1252/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1252<F: Float>(t3014: F, t40: F, t1933: F, t23479: F, t1004: F, t23528: F, t23544: F, t3053: F, t10948: F, t23536: F, t23437: F, t3103: F, t10472: F, t10474: F, t10478: F, t23535: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t83032 = t40 * t3014;
    let t83034 = t1933 * t83032 * t23479;
    let t83038 = t1004 * t23528;
    let t83041 = t23544 * t3053;
    let t83043 = t10948 * t23536;
    let t83046 = t23437 * t3103;
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    (t83032, t83034, t83038, t83041, t83043, t83046, t83054, t83058)
}
