//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 963/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk963<F: Float>(t118766: F, t30676: F, t5544: F, t6552: F, t6637: F, t23035: F, t5527: F, t118915: F, t118927: F, t118934: F, t118940: F, t1408: F, t7540: F) -> (F, F, F, F, F, F, F, F) {
    let t126484 = F::new(0.16449340668482264365e-1) * t118766;
    let t126488 = F::new(0.3289868133696452873e-1) * t6552 * t6637 * t30676 * t5544;
    let t126492 = F::new(0.9869604401089358619e-1) * t23035 * t6637 * t30676 * t5527;
    let t126497 = F::new(0.76763589786250567036e-1) * t118915;
    let t126518 = F::new(0.76763589786250567036e-1) * t118927;
    let t126520 = F::new(0.16449340668482264365e-1) * t118934;
    let t126521 = F::new(0.3289868133696452873e-1) * t118940;
    let t126530 = t1408 * t7540;
    (t126484, t126488, t126492, t126497, t126518, t126520, t126521, t126530)
}
