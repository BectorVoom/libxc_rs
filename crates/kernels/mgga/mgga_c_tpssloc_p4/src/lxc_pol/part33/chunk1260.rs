//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1260/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1260<F: Float>(t22764: F, t5234: F, t3862: F, t7715: F, t26245: F, t80791: F, t26271: F, t80836: F, t80783: F, t22760: F, t1827: F, t80914: F) -> (F, F, F, F, F, F, F) {
    let t91285 = t5234 * t22764;
    let t91305 = t7715 * t3862;
    let t91312 = t80791 * t26245;
    let t91323 = t80836 * t26271;
    let t91346 = t80783 * t26245;
    let t91388 = t5234 * t22760;
    let t91394 = t80914 * t1827;
    (t91285, t91305, t91312, t91323, t91346, t91388, t91394)
}
