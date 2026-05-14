//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1008/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1008<F: Float>(t27607: F, t460: F, t24682: F, t24658: F, t3: F, t24719: F, t3030: F, t1734: F, t3503: F, t1215: F, t24815: F, t1210: F, t1011: F, t475: F, t1218: F, t1232: F, t1737: F, t1748: F, t24685: F, t24712: F, t24716: F, t24736: F, t27604: F, t27609: F, t27611: F, t27614: F, t27617: F, t27622: F, t27626: F, t7331: F, t8040: F) -> (F,) {
    let t27628 = t27607 * t460;
    let t27629 = t24682 * t27628;
    let t27634 = t24658 * t3;
    let t27635 = t24719 * t3030;
    let t27636 = t27634 * t27635;
    let t27637 = t3503 * t1734;
    let t27638 = t24815 * t1215;
    let t27639 = t27637 * t27638;
    let t27642 = t1210 * t1734;
    let t27643 = t1011 * t1215;
    let t27644 = t27643 * t475;
    let t27645 = t27642 * t27644;
    let t27648 = t27604 * t1232 / 432.0 - 0.10093189023535097714e-3 * t27609 + t27611 / 2304.0 - 0.10093189023535097714e-3 * t24712 + t27614 * t1218 / 1536.0 - t27617 * t1232 / 2304.0 + t24716 * t1737 / 1536.0 - t27622 / 3456.0 - t24736 * t1748 / 2304.0 - t27626 / 864.0 - 0.10093189023535097714e-3 * t27629 * t7331 - 0.10093189023535097714e-3 * t24685 * t8040 + 0.20186378047070195428e-3 * t27636 * t27639 - 0.10093189023535097714e-3 * t27636 * t27645;
    (t27648,)
}
