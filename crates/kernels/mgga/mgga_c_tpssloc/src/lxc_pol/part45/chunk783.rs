//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 783/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk783<F: Float>(t344: F, t6729: F, t6740: F, t3008: F, t343: F, t6734: F, t3103: F, t6755: F, t3120: F, t360: F, t68: F, t6744: F) -> (F, F, F, F) {
    let t23488 = t6729 * t344;
    let t23489 = t6740 * t23488;
    let t23494 = t3008 * t343;
    let t23495 = t23494 * t6734;
    let t23500 = t6755 * t3103;
    let t23503 = t3120 * t68 * t360;
    let t23504 = t6744 * t23503;
    (t23489, t23495, t23500, t23504)
}
