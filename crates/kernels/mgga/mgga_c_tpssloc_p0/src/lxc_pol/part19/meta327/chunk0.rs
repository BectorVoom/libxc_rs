//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1162/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1162<F: Float>(t12250: F, t40045: F, t550: F, t1336: F, t2690: F, t3788: F, t3795: F, t3792: F, t67: F, t6924: F, t246: F, t12156: F) -> (F, F, F, F, F, F) {
    let t40148 = t40045 * t12250;
    let t40153 = t40045 * t550;
    let t40159 = t1336 * t3788 * t2690;
    let t40160 = t40159 * t3795;
    let t40162 = t40045 * t3792;
    let t40167 = t6924 * t67;
    let t40168 = t40167 * t246;
    let t40169 = t550 * t12156;
    (t40148, t40153, t40160, t40162, t40168, t40169)
}
