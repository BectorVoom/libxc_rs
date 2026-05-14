//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1070/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1070<F: Float>(t6876: F, t8641: F, t2075: F, t6534: F, t652: F, t671: F, t8595: F, t7000: F, t8607: F, t7057: F, t8526: F, t532: F, t8639: F, t6879: F, t1983: F, t2314: F, t8533: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31737 = t6876 * t8641;
    let t31744 = t2075 * t6534;
    let t31746 = 2.0 * t652 * t31744;
    let t31747 = t8595 * t671;
    let t31749 = 2.0 * t652 * t31747;
    let t31750 = t8607 * t7000;
    let t31753 = 2.0 * t8526 * t7057;
    let t31758 = t532 * t8639;
    let t31759 = t31758 * t6879;
    let t31761 = 3.0 * t1983 * t31759;
    let t31769 = 2.0 * t2314 * t8533;
    (t31737, t31744, t31746, t31747, t31749, t31750, t31753, t31758, t31759, t31761, t31769)
}
