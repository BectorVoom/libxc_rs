//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 891/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk891<F: Float>(t2770: F, t5398: F, t5689: F, t892: F, t3216: F, t5946: F, t10595: F, t5698: F, t10599: F, t5717: F, t699: F, t5720: F) -> (F, F, F, F, F, F, F) {
    let t17177 = t2770 * t5398;
    let t17195 = t5689 * t892;
    let t17202 = t5946 * t3216;
    let t17210 = t10595 * t5698;
    let t17218 = t10599 * t5698;
    let t17286 = t699 * t5717;
    let t17288 = t699 * t5720;
    (t17177, t17195, t17202, t17210, t17218, t17286, t17288)
}
