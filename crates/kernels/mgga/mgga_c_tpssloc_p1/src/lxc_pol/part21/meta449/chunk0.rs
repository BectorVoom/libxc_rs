//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1999/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1999<F: Float>(t15708: F, t4728: F, t3578: F, t1735: F, t3243: F, t11668: F, t1744: F, t3540: F, t1731: F, t1222: F, t4961: F, t1743: F, t3566: F) -> (F, F, F, F, F, F, F, F) {
    let t15709 = t4728 * t15708;
    let t15710 = t3578 * t15709;
    let t15713 = t1735 * t3243;
    let t15714 = t11668 * t15713;
    let t15717 = t1744 * t3540;
    let t15719 = t1731 * t3540;
    let t15722 = t4961 * t1222 / F::cast_from(432.0_f64);
    let t15723 = t3566 * t1743;
    (t15709, t15710, t15713, t15714, t15717, t15719, t15722, t15723)
}
