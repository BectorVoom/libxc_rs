//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 805/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk805<F: Float>(t761: F, t9905: F, t9820: F, t9824: F, t9881: F, t9884: F, t9887: F, t9890: F, t9894: F, t9896: F, t9900: F, t9903: F, t2250: F, t751: F, t707: F, t2447: F, t706: F) -> (F, F, F, F, F) {
    let t9907 = 0.35089341735807877242e1 * t761 * t9905;
    let t9908 = -t9820 - t9824 + t9881 - t9884 + t9887 + t9890 - t9894 + t9896 + t9900 - t9903 + t9907;
    let t9909 = t751 * t2250;
    let t9910 = t707 * t9909;
    let t9911 = 12.0 * t9910;
    let t9912 = t706 * t2447;
    (t9907, t9908, t9909, t9911, t9912)
}
