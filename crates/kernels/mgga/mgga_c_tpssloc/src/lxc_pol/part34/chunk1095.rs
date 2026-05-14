//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1095/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1095<F: Float>(t102137: F, t102139: F, t102142: F, t102173: F, t102267: F, t106836: F, t106862: F, t2032: F, t26911: F, t27966: F, t27972: F, t28935: F, t7432: F, t7435: F, t7782: F, t84216: F, t91905: F, t91922: F) -> (F,) {
    let t108727 = -2.0 * t7435 * t28935 - 70.0 * t84216 * t106836 - 5.0 * t102267 * t7432 - 2.0 * t106862 * t2032 - 10.0 * t26911 * t27972 - 4.0 * t27966 * t7782 - 16.0 / 3.0 * t102137 + 16.0 / 3.0 * t102139 - 8.0 / 3.0 * t102142 - 176.0 / 9.0 * t91905 - 440.0 / 9.0 * t91922 - 160.0 / 3.0 * t102173;
    (t108727,)
}
