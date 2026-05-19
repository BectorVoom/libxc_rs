//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 809/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk809<F: Float>(t1527: F, t7841: F, t2718: F, t10110: F, t2053: F, t5636: F, t2047: F, t5558: F, t1492: F, t7823: F, t1528: F, t17052: F, t17092: F, t2054: F, t24291: F, t24318: F, t24321: F, t25206: F, t25209: F, t25211: F, t25346: F, t259: F, t26700: F, t28440: F, t4147: F, t4268: F, t5658: F, t7087: F, t7842: F, t855: F) -> (F, F, F, F, F) {
    let t29079 = t7841 * t1527;
    let t29080 = t2718 * t29079;
    let t29091 = t10110 * t2053 * t5636;
    let t29095 = t5558 * t2047;
    let t29099 = t1492 * t7823;
    let t29104 = -t24291 - F::new(2.0) * t4147 * t7842 + F::cast_from(0.16449340668482264365e-1_f64) * t25206 + F::new(4.0) * t855 * t29080 - t7087 * t5658 + F::cast_from(0.15352717957250113407e0_f64) * t25209 + F::cast_from(0.76763589786250567036e-1_f64) * t25211 + t24318 + t24321 - F::new(2.0) * t26700 * t1528 - F::cast_from(0.3289868133696452873e-1_f64) * t28440 - t17052 * t2054 - F::new(6.0) * t855 * t29091 + F::cast_from(0.3289868133696452873e-1_f64) * t25346 + t29095 * t259 - F::new(2.0) * t17092 * t2054 + F::new(2.0) * t29099 * t259 - F::new(2.0) * t4268 * t7842;
    (t29080, t29091, t29095, t29099, t29104)
}
