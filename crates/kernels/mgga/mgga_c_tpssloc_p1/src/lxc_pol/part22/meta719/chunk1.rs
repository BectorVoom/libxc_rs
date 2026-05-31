//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2328/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2328<F: Float>(t13151: F, t13156: F, t13160: F, t1504: F, t16662: F, t16736: F, t16749: F, t16949: F, t20756: F, t20800: F, t20843: F, t20846: F, t20849: F, t228: F, t4119: F, t4225: F, t4226: F, t5544: F, t6589: F, t67282: F, t776: F, t822: F, t824: F, t845: F) -> F {
    let t67566 = -F::cast_from(360.0_f64) * t20756 * t4225 * t6589 * t776 - F::cast_from(12.0_f64) * t20800 * t4225 * t776 * t845 + F::cast_from(180.0_f64) * t13156 * t16949 * t4225 - F::cast_from(36.0_f64) * t13160 * t4225 * t5544 - F::cast_from(36.0_f64) * t16662 * t4225 * t4226 + F::cast_from(180.0_f64) * t16736 * t4119 * t4225 + F::cast_from(3.0_f64) * t228 * t67282 * t824 - F::cast_from(36.0_f64) * t13151 * t20846 + F::cast_from(9.0_f64) * t1504 * t16749 + F::cast_from(60.0_f64) * t20843 * t822 + F::cast_from(3.0_f64) * t20849 * t822;
    t67566
}
