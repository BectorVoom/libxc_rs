//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1094/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1094<F: Float>(t1667: F, t9709: F, t11274: F, t1657: F, t11189: F, t11282: F, t1687: F, t11419: F, t1675: F, t11349: F, t11292: F, t1714: F, t44583: F, t15418: F, t1174: F, t1716: F, t2402: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t50846 = t9709 * t1667;
    let t51120 = t1657 * t11274;
    let t51249 = t1657 * t11189;
    let t51376 = t1687 * t11282;
    let t51427 = t1675 * t11419;
    let t51604 = t1675 * t11349;
    let t51680 = t1687 * t11292;
    let t51968 = t44583 * t1714;
    let t52059 = t15418 * t1714;
    let t52081 = t1174 * t2402 * t1716;
    (t50846, t51120, t51249, t51376, t51427, t51604, t51680, t51968, t52059, t52081)
}
