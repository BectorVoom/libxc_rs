//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1147/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1147<F: Float>(t3082: F, t8388: F, t1004: F, t30839: F, t3127: F, t368: F, t1030: F, t354: F, t362: F, t372: F, t23384: F, t30855: F) -> (F, F, F, F, F) {
    let t113432 = t8388 * t3082 / F::new(6912.0);
    let t113438 = t1004 * t30839;
    let t113443 = t3127 * t368;
    let t113454 = t354 * t362 * t1030 * t372;
    let t113468 = t23384 * t30855;
    (t113432, t113438, t113443, t113454, t113468)
}
