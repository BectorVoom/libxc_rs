//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 753/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk753<F: Float>(t3844: F, t885: F, t1436: F, t2577: F, t884: F, t2455: F, t2581: F, t3746: F, t3751: F, t3756: F, t3760: F, t318: F, t1441: F, t895: F, t1449: F, t903: F) -> (F, F, F, F, F, F, F) {
    let t3845 = t3844 * t885;
    let t3848 = t1436 * t2577;
    let t3849 = t3848 * t884;
    let t3857 = t2581 + 0.30902777777777777778e-2 * t2455 + 0.30902777777777777778e-2 * t3746 - 0.61805555555555555555e-2 * t3751 + 0.18541666666666666667e-1 * t3756 - 0.92708333333333333333e-2 * t3760;
    let t3858 = t3857 * t318;
    let t3860 = t1441 * t895;
    let t3865 = t1449 * t903;
    (t3845, t3848, t3849, t3857, t3858, t3860, t3865)
}
