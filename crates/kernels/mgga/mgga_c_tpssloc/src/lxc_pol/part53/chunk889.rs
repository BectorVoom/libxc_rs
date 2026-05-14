//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 889/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk889<F: Float>(t1992: F, t550: F, t6976: F, t93505: F, t33285: F, t6883: F, t33284: F, t6897: F, t794: F, t22897: F, t27075: F, t27078: F, t122448: F, t1352: F, t22633: F, t27074: F, t3807: F) -> (F, F, F, F, F, F, F) {
    let t122488 = t1992 * t6976 * t93505 * t550;
    let t122503 = t6883 * t33285;
    let t122507 = t6897 * t794 * t33284;
    let t122510 = t1992 * t22897 * t27075;
    let t122513 = t1992 * t6976 * t27078;
    let t122518 = t22633 * t6976 * t122448 * t1352;
    let t122522 = t22633 * t6976 * t27074 * t3807;
    (t122488, t122503, t122507, t122510, t122513, t122518, t122522)
}
