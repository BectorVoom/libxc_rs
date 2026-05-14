//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 787/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk787<F: Float>(t1137: F, t4819: F, t1682: F, t3359: F, t1136: F, t3238: F, t3363: F, t4721: F, t4726: F, t4731: F, t4735: F, t449: F, t1147: F, t1687: F, t1155: F, t1695: F) -> (F, F, F, F, F, F, F) {
    let t4820 = t4819 * t1137;
    let t4823 = t1682 * t3359;
    let t4824 = t4823 * t1136;
    let t4832 = t3363 - 0.30902777777777777778e-2 * t3238 - 0.30902777777777777778e-2 * t4721 - 0.61805555555555555555e-2 * t4726 + 0.18541666666666666667e-1 * t4731 + 0.92708333333333333333e-2 * t4735;
    let t4833 = t4832 * t449;
    let t4835 = t1687 * t1147;
    let t4840 = t1695 * t1155;
    (t4820, t4823, t4824, t4832, t4833, t4835, t4840)
}
