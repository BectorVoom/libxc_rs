//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2014/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2014<F: Float>(t22716: F, t6903: F, t22662: F, t22674: F, t6897: F, t22684: F, t6546: F, t22687: F, t131: F, t1365: F, t22648: F, t794: F) -> (F, F, F, F, F, F) {
    let t80722 = t22716 * t6903;
    let t80725 = t6897 * t22674 * t22662;
    let t80727 = t6546 * t22684;
    let t80728 = t80727 * t22687;
    let t80730 = t1365 * t131;
    let t80738 = t6897 * t794 * t22648;
    (t80722, t80725, t80727, t80728, t80730, t80738)
}
