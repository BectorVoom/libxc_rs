//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 257/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk257<F: Float>(t1574: F, t300: F, t1580: F, t942: F, t951: F, t959: F, t1409: F, t978: F, t977: F, t1554: F, t906: F, t340: F, t343: F, t974: F, t971: F, t973: F) -> (F, F, F, F, F, F, F) {
    let t1587 = 0.19751673498613801407e-1 * t300 * t1574;
    let t1589 = t942 * t1580 * t951;
    let t1591 = 0.5848223622634646207e0 * t959 * t1589;
    let t1592 = t978 * t1409;
    let t1593 = t977 * t1592;
    let t1597 = t906 / 6.0 + t1554 / 6.0;
    let t1598 = t340 * t1597;
    let t1599 = t1598 * t343;
    let t1600 = t974 * t1599;
    let t1603 = t971 + 0.27777777777777777777e-3 * t973 * t1593 - 0.83333333333333333332e-3 * t973 * t1600;
    (t1587, t1589, t1591, t1592, t1597, t1599, t1603)
}
