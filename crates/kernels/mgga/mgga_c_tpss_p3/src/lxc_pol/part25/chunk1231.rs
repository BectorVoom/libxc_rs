//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1231/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1231<F: Float>(t1707: F, t17993: F, t18006: F, t19767: F, t20471: F, t20475: F, t20479: F, t20483: F, t20488: F, t20492: F, t20494: F, t20498: F, t20503: F, t20506: F, t5568: F, t5571: F, t6348: F, t6351: F) -> F {
    let t20508 = -t1707 * t20506 + t17993 * t6348 - F::cast_from(2.0_f64) * t18006 * t20479 - F::cast_from(2.0_f64) * t19767 * t20483 + t19767 * t20494 + F::cast_from(2.0_f64) * t20471 * t5571 + F::cast_from(2.0_f64) * t20475 * t5571 + t20488 * t5571 + t20492 * t5571 + F::cast_from(2.0_f64) * t20498 * t5571 + t20503 * t5571 - t5568 * t6351;
    t20508
}
