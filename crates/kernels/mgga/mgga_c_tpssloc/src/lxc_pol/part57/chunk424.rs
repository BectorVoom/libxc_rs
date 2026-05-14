//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 424/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk424<F: Float>(t5617: F, t819: F, t820: F, t2701: F, t5527: F, t5544: F, t847: F, t1512: F, t1516: F, t249: F, t2571: F, t2602: F, t2630: F, t2643: F, t2695: F, t4152: F, t4167: F, t4170: F, t4172: F, t4187: F, t4253: F, t5568: F, t5572: F, t5576: F, t5587: F, t5593: F, t5614: F, t787: F, t817: F, t843: F) -> (F, F, F, F) {
    let t5619 = t819 * t820 * t5617;
    let t5624 = t2701 * t820 * t5527;
    let t5628 = t847 * t820 * t5544;
    let t5631 = t2602 + 7.0 / 72.0 * t4152 + t2571 * t5568 / 16.0 - t787 * t5572 / 48.0 + t5576 * t249 / 3072.0 - t4167 * t1512 / 1536.0 - 7.0 / 2304.0 * t4170 - t4172 * t1516 / 384.0 + t2630 * t5587 / 1536.0 + 7.0 / 2304.0 * t4187 + t2643 * t5593 / 384.0 - t817 * t5614 / 3072.0 - t817 * t5619 / 3072.0 + t2695 + 7.0 / 576.0 * t4253 + 5.0 / 768.0 * t843 * t5624 - t843 * t5628 / 768.0;
    (t5619, t5624, t5628, t5631)
}
