//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 951/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk951<F: Float>(t19451: F, t8326: F, t28002: F, t1458: F, t7450: F, t1868: F, t5493: F, t1484: F, t7540: F, t22960: F, t25: F, t28447: F) -> (F, F, F, F, F, F, F) {
    let t126118 = F::new(2.0) * t19451 * t8326;
    let t126120 = F::new(4.0) * t28002 * t8326;
    let t126127 = t7450 * t1458;
    let t126132 = t1868 * t5493;
    let t126176 = t1484 * t7540;
    let t126177 = t22960 * t126176;
    let t126180 = t25 * t28447;
    (t126118, t126120, t126127, t126132, t126176, t126177, t126180)
}
