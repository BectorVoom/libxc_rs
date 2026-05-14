//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 820/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk820<F: Float>(t21318: F, t2842: F, t1569: F, t5758: F, t10636: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F, t291: F) -> (F, F, F) {
    let t21320 = 0.48245938496077605201e2 * t2842 * t21318;
    let t21321 = t1569 * t5758;
    let t21334 = -t10636 - 0.23744444444444444444e-1 * t13598 + 0.11872222222222222222e-1 * t17149 - 0.35616666666666666666e-1 * t17165 + 0.17808333333333333333e-1 * t17175 - 0.19787037037037037037e-1 * t21147 + 0.71233333333333333332e-1 * t21150 - 0.35616666666666666666e-1 * t21124 - 0.10685e0 * t21153 + 0.10685e0 * t21128 - 0.17808333333333333333e-1 * t21156;
    let t21336 = 0.621814e-1 * t21334 * t291;
    (t21320, t21321, t21336)
}
