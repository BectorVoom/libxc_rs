//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1036/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1036<F: Float>(t11137: F, t14818: F, t18227: F, t18239: F, t18497: F, t18500: F, t18503: F, t18508: F, t18510: F, t18515: F, t18518: F, t11314: F, t14722: F, t14766: F, t15083: F, t15094: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18229: F, t18234: F, t18243: F, t18494: F, t18505: F, t18512: F, t18521: F, t18731: F, t18762: F, t18869: F) -> (F,) {
    let t18891 = 0.20659e1 * t18227 + 0.4630888888888888889e-1 * t14818 - 0.34731666666666666667e-1 * t18515 + 0.46308888888888888889e-1 * t18497 + 0.20839e0 * t18518 + 0.22954444444444444444e0 * t11137 + 0.103295e1 * t18239 - 0.69463333333333333334e-1 * t18503 - 0.20839e0 * t18500 + 0.41678e0 * t18510 + 0.62517e0 * t18508;
    let t18893 = 0.3529725e1 * t18731 - t11314 - 0.69463333333333333333e-1 * t18512 + 0.104195e0 * t18521 + 0.11477222222222222222e0 * t18203 - 0.34431666666666666667e0 * t18219 - 0.17215833333333333333e0 * t18229 + 0.516475e0 * t18243 + 0.23154444444444444445e-1 * t18494 - 0.13892666666666666667e0 * t18505 + t18869 - 0.17648625e1 * t18762 + 0.23154444444444444445e0 * t14766 + t15083 - 0.68863333333333333332e0 * t14722 - t15094 - 0.34431666666666666667e0 * t18234 + 0.57386111111111111112e0 * t18208 - 0.20659e1 * t18213 - 0.68863333333333333334e0 * t18217 + 0.309885e1 * t18223 + t18891;
    (t18893,)
}
