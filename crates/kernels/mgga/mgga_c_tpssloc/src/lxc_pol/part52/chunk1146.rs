//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1146/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1146<F: Float>(t1894: F, t4119: F, t59: F, t6591: F, t30714: F, t4240: F, t4250: F, t4191: F, t112818: F, t112820: F, t112829: F, t112835: F, t112841: F, t112846: F, t112851: F, t112856: F, t118586: F, t118588: F, t118590: F, t118592: F, t118594: F, t118596: F, t118602: F) -> (F,) {
    let t118606 = t6591 * t1894 * t59 * t4119;
    let t118608 = t30714 * t4240;
    let t118610 = t30714 * t4250;
    let t118612 = t30714 * t4191;
    let t118615 = 0.13457585364713463618e-3 * t118586 + 7.0 / 576.0 * t118588 - t118590 / 384.0 - t118592 / 384.0 - t118594 / 384.0 + 7.0 / 2304.0 * t118596 + 0.80745512188280781708e-3 * t112818 + 7.0 / 576.0 * t112820 + 0.56521858531796547196e-2 * t112829 - 7.0 / 2304.0 * t118602 + t112835 - t112841 - 0.48447307312968469025e-2 * t118606 - t118608 / 1536.0 + t118610 / 384.0 + t118612 / 384.0 - 7.0 / 2304.0 * t112846 + t112851 + t112856;
    (t118615,)
}
