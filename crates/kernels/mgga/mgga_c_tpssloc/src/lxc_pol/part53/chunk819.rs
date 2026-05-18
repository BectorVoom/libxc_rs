//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 819/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk819<F: Float>(t26414: F, t6976: F, t22633: F, t5345: F, t1992: F, t1799: F, t562: F, t1352: F, t22705: F, t7736: F, t22704: F, t6883: F, t7741: F) -> (F, F, F, F, F, F) {
    let t26415 = t6976 * t26414;
    let t26416 = t22633 * t26415;
    let t26418 = t6976 * t5345;
    let t26419 = t1992 * t26418;
    let t26421 = t562 * t1799;
    let t26422 = t26421 * t1352;
    let t26423 = t6976 * t26422;
    let t26424 = t22633 * t26423;
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    let t26429 = t6883 * t7741;
    (t26416, t26419, t26421, t26424, t26427, t26429)
}
