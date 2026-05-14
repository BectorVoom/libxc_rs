//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 837/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk837<F: Float>(t6733: F, t6743: F, t6796: F, t995: F, t6802: F, t614: F, t6794: F, t131: F, t350: F, t1011: F, t3131: F, t1049: F, t362: F, t23384: F, t6787: F, t3216: F, t6818: F) -> (F, F, F, F, F, F, F, F) {
    let t23657 = t6733 * t6743;
    let t23665 = t6796 * t995;
    let t23666 = t23665 * t6802;
    let t23668 = t614 * t6794;
    let t23669 = t23668 * t131;
    let t23670 = t23669 * t350;
    let t23678 = t1011 * t3131;
    let t23685 = t362 * t1049;
    let t23712 = t23384 * t6787;
    let t23738 = t6818 * t3216;
    (t23657, t23665, t23666, t23670, t23678, t23685, t23712, t23738)
}
