//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1923/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1923<F: Float>(t6420: F, t6987: F, t1825: F, t26458: F, t19743: F, t550: F, t6976: F, t1992: F, t1336: F, t22718: F, t22726: F, t26437: F, t27096: F, t28156: F, t28161: F, t28165: F, t28169: F, t28171: F, t5234: F, t544: F, t7745: F) -> (F, F, F, F, F) {
    let t28174 = t6987 * t6420;
    let t28178 = t26458 * t1825;
    let t28181 = t19743 * t550;
    let t28182 = t6976 * t28181;
    let t28183 = t1992 * t28182;
    let t28185 = t544 * t28156 - t27096 - F::cast_from(0.82246703342411321824e-2_f64) * t26437 + F::cast_from(0.82246703342411321825e-2_f64) * t28161 + t22718 + t22726 - F::cast_from(0.16449340668482264365e-1_f64) * t28165 - F::cast_from(0.82246703342411321825e-2_f64) * t28169 + F::new(2.0) * t1336 * t28171 - t1336 * t28174 - F::new(2.0) * t5234 * t7745 - F::new(2.0) * t1336 * t28178 - F::cast_from(0.82246703342411321825e-2_f64) * t28183;
    (t28174, t28178, t28181, t28182, t28185)
}
