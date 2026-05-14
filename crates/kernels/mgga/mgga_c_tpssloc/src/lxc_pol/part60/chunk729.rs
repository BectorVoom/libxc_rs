//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 729/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk729<F: Float>(t28830: F, t8643: F, t22574: F, t1390: F, t6347: F, t6878: F, t1983: F, t7685: F, t7688: F, t7754: F, t19596: F, t2019: F, t7458: F, t7468: F, t1873: F, t6287: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28831 = t8643 * t28830;
    let t28833 = 6.0 * t22574 * t28831;
    let t28834 = t1390 * t6347;
    let t28835 = t6878 * t28834;
    let t28837 = 3.0 * t1983 * t28835;
    let t28841 = 6.0 * t7685 * t7688;
    let t28843 = 2.0 * t7685 * t7754;
    let t28860 = t2019 * t19596;
    let t28861 = t1983 * t28860;
    let t28863 = 4.0 * t7458 * t7468;
    let t28864 = t6287 * t1873;
    (t28831, t28833, t28834, t28835, t28837, t28841, t28843, t28860, t28861, t28863, t28864)
}
