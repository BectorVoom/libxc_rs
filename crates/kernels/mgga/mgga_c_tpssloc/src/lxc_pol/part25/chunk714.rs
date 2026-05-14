//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 714/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk714<F: Float>(t707: F, t9909: F, t2447: F, t706: F, t708: F, t157: F, t9448: F, t182: F, t2509: F, t746: F, t9490: F, t761: F, t2531: F, t2535: F, t2427: F, t2430: F) -> (F, F, F, F, F, F, F) {
    let t9910 = t707 * t9909;
    let t9911 = 12.0 * t9910;
    let t9912 = t706 * t2447;
    let t9914 = 12.0 * t9912 * t708;
    let t9915 = t9448 * t157;
    let t9917 = 0.19751673498613801407e-1 * t9915 * t182;
    let t9919 = t2509 * t9490 * t746;
    let t9921 = 0.35089341735807877242e1 * t761 * t9919;
    let t9922 = t2531 * t2535;
    let t9923 = 0.17544670867903938621e1 * t9922;
    let t9924 = t2427 * t2430;
    (t9911, t9914, t9917, t9919, t9921, t9923, t9924)
}
