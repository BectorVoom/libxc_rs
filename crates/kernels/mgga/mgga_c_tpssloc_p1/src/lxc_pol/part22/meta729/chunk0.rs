//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2390/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2390<F: Float>(t48157: F, t60192: F, t60194: F, t60202: F, t68571: F, t68577: F, t68580: F, t68583: F, t68586: F, t68589: F, t68592: F, t10564: F, t123: F, t68521: F) -> (F, F) {
    let t68594 = -F::cast_from(0.91983333333333333333e-1_f64) * t48157 - F::new(0.301925e0) * t68571 + F::new(0.99342e0) * t60192 - F::new(0.66228e0) * t60194 - F::new(0.33114e0) * t60202 + F::new(0.72462e1) * t68577 - F::new(0.543465e1) * t68580 + F::new(0.181155e1) * t68583 + F::new(0.181155e1) * t68586 + F::new(0.60385e0) * t68589 - F::cast_from(0.20128333333333333333e0_f64) * t68592;
    let t68596 = t123 * t10564 * t68521;
    (t68594, t68596)
}
