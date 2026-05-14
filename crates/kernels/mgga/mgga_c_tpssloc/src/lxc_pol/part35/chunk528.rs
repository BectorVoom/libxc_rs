//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 528/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk528<F: Float>(t1409: F, t2770: F, t2775: F, t1543: F, t892: F, t1547: F, t2798: F, t2815: F, t1553: F, t699: F, t1561: F, t923: F, t1573: F, t942: F, t1580: F, t2932: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4337 = t2770 * t1409;
    let t4342 = t2775 * t1409;
    let t4354 = t1543 * t892;
    let t4362 = t2798 * t1547;
    let t4378 = t2815 * t1547;
    let t4384 = t699 * t1553;
    let t4411 = t1561 * t923;
    let t4449 = t1573 * t942;
    let t4475 = t1580 * t2932;
    (t4337, t4342, t4354, t4362, t4378, t4384, t4411, t4449, t4475)
}
