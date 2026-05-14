//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 619/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk619<F: Float>(t3758: F, t835: F, t128: F, t2454: F, t2455: F, t3746: F, t3751: F, t3756: F, t285: F, t1411: F, t845: F, t867: F, t1425: F, t2476: F, t865: F, t2481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3759 = t835 * t3758;
    let t3760 = t128 * t3759;
    let t3762 = t2454 + 0.5936111111111111111e-2 * t2455 + 0.5936111111111111111e-2 * t3746 - 0.11872222222222222222e-1 * t3751 + 0.35616666666666666666e-1 * t3756 - 0.17808333333333333333e-1 * t3760;
    let t3764 = 0.621814e-1 * t3762 * t285;
    let t3765 = t1411 * t845;
    let t3767 = 1.0 * t3765 * t867;
    let t3769 = 1.0 * t2476 * t1425;
    let t3770 = t1425 * t865;
    let t3772 = 2.0 * t2481 * t3770;
    (t3759, t3760, t3762, t3764, t3765, t3767, t3769, t3770, t3772)
}
