//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 623/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk623<F: Float>(t2455: F, t2513: F, t2557: F, t2564: F, t3746: F, t3751: F, t3756: F, t3760: F, t3774: F, t3782: F, t3790: F, t3792: F, t3795: F, t3798: F, t3801: F, t3804: F) -> (F,) {
    let t3844 = -0.17648625e1 * t3774 + 0.3529725e1 * t3782 + t2557 + 0.17215833333333333333e0 * t2455 + 0.17215833333333333333e0 * t3746 - 0.34431666666666666667e0 * t3751 + 0.103295e1 * t3756 - 0.516475e0 * t3760 + 0.31558125e0 * t3790 + 0.6311625e0 * t3792 + t2564 + 0.69463333333333333333e-1 * t2513 + 0.69463333333333333333e-1 * t3795 - 0.34731666666666666667e-1 * t3798 + 0.20839e0 * t3801 - 0.104195e0 * t3804;
    (t3844,)
}
