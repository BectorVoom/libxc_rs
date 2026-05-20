//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1038/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1038<F: Float>(t1985: F, t26193: F, t33296: F, t127430: F, t22633: F, t22635: F, t31558: F, t122124: F, t1799: F, t1992: F, t26989: F, t6439: F) -> (F, F, F, F) {
    let t128797 = t1985 * t26193 * t33296;
    let t128805 = t22633 * t22635 * t31558 * t127430;
    let t128809 = t22633 * t22635 * t122124 * t1799;
    let t128816 = t1992 * t22635 * t26989 * t6439;
    (t128797, t128805, t128809, t128816)
}
