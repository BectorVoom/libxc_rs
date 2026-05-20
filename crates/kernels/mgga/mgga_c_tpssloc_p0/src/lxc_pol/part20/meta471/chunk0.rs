//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1940/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1940<F: Float>(t15700: F, t15702: F, t3578: F, t1215: F, t607: F, t475: F, t4728: F, t1735: F, t3243: F, t11668: F, t1744: F, t3540: F) -> (F, F, F, F, F, F, F, F) {
    let t15703 = t15700 * t15702;
    let t15704 = t3578 * t15703;
    let t15707 = t607 * t1215;
    let t15708 = t15707 * t475;
    let t15709 = t4728 * t15708;
    let t15710 = t3578 * t15709;
    let t15713 = t1735 * t3243;
    let t15714 = t11668 * t15713;
    let t15717 = t1744 * t3540;
    (t15703, t15704, t15708, t15709, t15710, t15713, t15714, t15717)
}
