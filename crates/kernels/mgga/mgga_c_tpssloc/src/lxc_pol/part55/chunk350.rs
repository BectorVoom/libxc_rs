//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 350/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk350<F: Float>(t1580: F, t951: F, t1545: F, t1559: F, t1561: F, t1569: F, t1574: F, t300: F, t311: F, t924: F, t943: F, t942: F, t959: F, t1409: F, t978: F, t977: F) -> (F, F, F, F, F, F, F) {
    let t1581 = t1580 * t951;
    let t1585 = t300 * (-0.310907e-1 * t1561 * t311 + 1.0 * t924 * t1569 + t1545 - t1559 - 0.19751673498613801407e-1 * t1574 + 0.5848223622634646207e0 * t943 * t1581);
    let t1587 = 0.19751673498613801407e-1 * t300 * t1574;
    let t1589 = t942 * t1580 * t951;
    let t1591 = 0.5848223622634646207e0 * t959 * t1589;
    let t1592 = t978 * t1409;
    let t1593 = t977 * t1592;
    (t1581, t1585, t1587, t1589, t1591, t1592, t1593)
}
