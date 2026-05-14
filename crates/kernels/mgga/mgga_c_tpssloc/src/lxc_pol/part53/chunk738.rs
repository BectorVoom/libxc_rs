//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 738/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk738<F: Float>(t1352: F, t26421: F, t6976: F, t22633: F, t22705: F, t7736: F, t22704: F, t6883: F, t7741: F, t1998: F, t5318: F, t214: F, t1985: F, t7740: F, t794: F, t6897: F) -> (F, F, F, F, F) {
    let t26422 = t26421 * t1352;
    let t26423 = t6976 * t26422;
    let t26424 = t22633 * t26423;
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    let t26429 = t6883 * t7741;
    let t26432 = t1998 * t5318;
    let t26433 = t214 * t26432;
    let t26434 = t1985 * t26433;
    let t26436 = t794 * t7740;
    let t26437 = t6897 * t26436;
    (t26424, t26427, t26429, t26434, t26437)
}
