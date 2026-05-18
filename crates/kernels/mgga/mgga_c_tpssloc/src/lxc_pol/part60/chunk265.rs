//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 265/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk265<F: Float>(t1573: F, t324: F, t1541: F, t1548: F, t1551: F, t1554: F, t945: F, t948: F, t951: F, t1545: F, t1559: F, t1561: F, t1569: F, t300: F, t311: F, t924: F, t943: F) -> (F, F, F, F) {
    let t1574 = t1573 * t324;
    let t1580 = F::new(0.258925e1) * t1548 - t945 - F::new(0.301925e0) * t1541 + F::new(0.16504875e0) * t1551 - t948 - F::new(0.82785e-1) * t1554;
    let t1581 = t1580 * t951;
    let t1585 = t300 * (-F::new(0.310907e-1) * t1561 * t311 + F::new(1.0) * t924 * t1569 + t1545 - t1559 - F::new(0.19751673498613801407e-1) * t1574 + F::new(0.5848223622634646207e0) * t943 * t1581);
    (t1574, t1580, t1581, t1585)
}
