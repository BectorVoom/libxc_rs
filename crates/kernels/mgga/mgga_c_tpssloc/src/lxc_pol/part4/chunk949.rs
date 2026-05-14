//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 949/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk949<F: Float>(t10817: F, t5695: F, t2787: F, t5727: F, t10296: F, t10556: F, t10675: F, t10676: F, t13551: F, t13552: F, t13563: F, t13567: F, t17173: F, t17180: F, t17185: F, t13598: F, t13650: F, t17149: F, t17165: F, t17175: F, t17189: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F) -> (F, F, F, F) {
    let t17377 = 2.0 * t10817 * t5695;
    let t17379 = 1.0 * t2787 * t5727;
    let t17398 = 0.11958666666666666667e1 * t17173 - t13551 + 0.36514074074074074073e-1 * t13552 + 0.13287407407407407407e0 * t13563 - t13567 - 0.91285185185185185187e-1 * t10296 - t10675 - t10676 - 0.19931111111111111111e0 * t17180 + 0.59793333333333333334e0 * t17185 - 0.13287407407407407408e0 * t10556;
    let t17420 = -0.26574814814814814815e0 * t13598 + t13650 + 0.16431333333333333333e0 * t17280 + 0.66437037037037037037e-1 * t17149 - 0.19931111111111111111e0 * t17165 + 0.99655555555555555557e-1 * t17175 - 0.29896666666666666667e0 * t17189 + 0.18257037037037037037e-1 * t17286 - 0.10954222222222222222e0 * t17288 + 0.54771111111111111111e-1 * t17290 - 0.82156666666666666667e-1 * t17293;
    (t17377, t17379, t17398, t17420)
}
