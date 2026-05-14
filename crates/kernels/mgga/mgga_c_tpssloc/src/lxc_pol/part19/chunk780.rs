//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 780/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk780<F: Float>(t2628: F, t835: F, t812: F, t2635: F, t2690: F, t815: F, t831: F, t2617: F, t2638: F, t2639: F, t2681: F, t184: F, t2250: F, t607: F, t4194: F, t116: F, t126: F, t136: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9668 = t9667 * t2635;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    let t9674 = t2617 * t2638;
    let t9675 = t9674 * t831;
    let t9679 = t2639 * t2681;
    let t9681 = t184 * t2250;
    let t9682 = t9681 * t607;
    let t9684 = 36.0 * t4194 * t9682;
    let t9688 = 1.0 / t126 / t136 * t116 / 4.0;
    (t9666, t9667, t9668, t9670, t9671, t9672, t9674, t9675, t9679, t9681, t9682, t9684, t9688)
}
