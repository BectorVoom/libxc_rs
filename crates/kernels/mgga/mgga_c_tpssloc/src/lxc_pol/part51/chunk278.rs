//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 278/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk278<F: Float>(t435: F, t1136: F, t1086: F, t1092: F, t449: F, t445: F) -> (F, F, F, F, F, F, F) {
    let t1137 = 1.0 / t435;
    let t1138 = t1136 * t1137;
    let t1141 = 0.92708333333333333333e-2 * t1086;
    let t1143 = -t1141 + 0.92708333333333333333e-2 * t1092;
    let t1144 = t1143 * t449;
    let t1146 = t445 * t445;
    let t1147 = 1.0 / t1146;
    (t1137, t1138, t1141, t1143, t1144, t1146, t1147)
}
