//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1099/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1099<F: Float>(t15904: F, t8643: F, t22574: F, t3701: F, t3914: F, t2019: F, t1983: F, t6996: F, t6999: F, t1390: F, t3719: F, t6878: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22575 = t8643 * t15904;
    let t22577 = F::cast_from(6.0_f64) * t22574 * t22575;
    let t22578 = t3701 * t3914;
    let t22579 = t2019 * t22578;
    let t22580 = t1983 * t22579;
    let t22581 = t6996 * t6999;
    let t22583 = F::cast_from(2.0_f64) * t1983 * t22581;
    let t22584 = t1390 * t3719;
    let t22585 = t6878 * t22584;
    (t22575, t22577, t22578, t22579, t22580, t22581, t22583, t22584, t22585)
}
