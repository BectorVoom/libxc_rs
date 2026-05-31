//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1510/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1510<F: Float>(t1347: F, t1819: F, t1821: F, t19708: F, t19715: F, t20416: F, t20536: F, t20544: F, t20547: F, t20550: F, t225: F, t3843: F, t40253: F, t5278: F, t5279: F, t546: F, t548: F, t6347: F, t6404: F, t6408: F, t6411: F, t79921: F, t79984: F, t80021: F, t80101: F, t80102: F, t80104: F, t80105: F, t80108: F, t80109: F, t80111: F, t80117: F) -> F {
    let t80150 = -(t80101 + t80102 + t80104 + t80105 + t80108 + t80109 + t80111 + t80117) * t225 * t548 + F::cast_from(12.0_f64) * t20536 * t1821 - F::cast_from(72.0_f64) * t6404 * t6408 + F::cast_from(18.0_f64) * t6404 * t6411 + F::cast_from(240.0_f64) * t1819 * t20544 - F::cast_from(144.0_f64) * t19708 * t20547 + F::cast_from(12.0_f64) * t1819 * t20550 - F::cast_from(360.0_f64) * t546 * t40253 * t80021 + F::cast_from(360.0_f64) * t5278 * t19715 * t6347 - F::cast_from(36.0_f64) * t546 * t3843 * t79921 - F::cast_from(48.0_f64) * t5278 * t5279 * t20416 + F::cast_from(3.0_f64) * t546 * t1347 * t79984;
    t80150
}
