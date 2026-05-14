//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1297/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1297<F: Float>(t1760: F, t1778: F, t41867: F, t507: F, t6273: F, t18539: F, t1270: F, t12810: F, t5708: F, t17898: F, t6243: F, t1689: F, t41905: F, t42719: F, t13133: F, t5522: F) -> (F, F, F, F, F, F, F) {
    let t65489 = t1760 * t1778 * t41867;
    let t65497 = t507 * t6273;
    let t65500 = 6.0 * t1760 * t65497 * t18539;
    let t65501 = t1270 * t12810;
    let t65504 = 3.0 * t1760 * t5708 * t65501;
    let t65506 = 2.0 * t6243 * t17898;
    let t65508 = 2.0 * t41905 * t1689;
    let t65510 = 4.0 * t42719 * t1689;
    let t65512 = 4.0 * t13133 * t5522;
    (t65489, t65500, t65504, t65506, t65508, t65510, t65512)
}
