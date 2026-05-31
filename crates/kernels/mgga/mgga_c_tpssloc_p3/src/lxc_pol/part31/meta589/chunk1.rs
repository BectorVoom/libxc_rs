//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1833/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1833<F: Float>(t3862: F, t7715: F, t22705: F, t22852: F, t236: F, t5286: F, t550: F, t26245: F, t80791: F, t80867: F, t26271: F, t80836: F) -> (F, F, F, F, F) {
    let t91305 = t7715 * t3862;
    let t91310 = t22852 * t22705 * t236 * t5286 * t550;
    let t91312 = t80791 * t26245;
    let t91314 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t80867;
    let t91323 = t80836 * t26271;
    (t91305, t91310, t91312, t91314, t91323)
}
