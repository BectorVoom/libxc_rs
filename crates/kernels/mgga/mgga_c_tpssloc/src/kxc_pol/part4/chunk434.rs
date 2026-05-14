//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 434/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk434<F: Float>(t1499: F, t237: F, t1464: F, t1473: F, t1476: F, t225: F, t680: F, t705: F, t752: F, t760: F, t765: F) -> (F, F) {
    let t1500 = t1499 * t237;
    let t1504 = (t680 + t705 + t1464 + t1473 + t752 + t1476 - t760 - t765) * t225;
    (t1500, t1504)
}
