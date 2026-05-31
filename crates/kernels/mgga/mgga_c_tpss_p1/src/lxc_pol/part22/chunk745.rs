//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 745/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk745<F: Float>(t1429: F, t876: F, t1437: F, t884: F, t2455: F, t2513: F, t2557: F, t2564: F, t3746: F, t3751: F, t3756: F, t3760: F, t3774: F, t3782: F, t3790: F, t3792: F, t3795: F, t3798: F, t3801: F, t3804: F) -> (F, F, F) {
    let t3822 = t1429 * t876;
    let t3827 = t1437 * t884;
    let t3844 = -F::cast_from(0.17648625e1_f64) * t3774 + F::cast_from(0.3529725e1_f64) * t3782 + t2557 + F::cast_from(0.17215833333333333333e0_f64) * t2455 + F::cast_from(0.17215833333333333333e0_f64) * t3746 - F::cast_from(0.34431666666666666667e0_f64) * t3751 + F::cast_from(0.103295e1_f64) * t3756 - F::cast_from(0.516475e0_f64) * t3760 + F::cast_from(0.31558125e0_f64) * t3790 + F::cast_from(0.6311625e0_f64) * t3792 + t2564 + F::cast_from(0.69463333333333333333e-1_f64) * t2513 + F::cast_from(0.69463333333333333333e-1_f64) * t3795 - F::cast_from(0.34731666666666666667e-1_f64) * t3798 + F::cast_from(0.20839e0_f64) * t3801 - F::cast_from(0.104195e0_f64) * t3804;
    (t3822, t3827, t3844)
}
