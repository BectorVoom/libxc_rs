//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2184/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2184<F: Float>(t22633: F, t26338: F, t90566: F, t22751: F, t28213: F, t28210: F, t28233: F, t6883: F, t1323: F, t16439: F, t19804: F, t2006: F, t22656: F, t28107: F, t28187: F, t3882: F, t568: F, t6361: F, t6461: F, t6955: F, t7750: F, t81284: F, t90702: F, t90708: F) -> (F, F, F) {
    let t97527 = t22633 * t90566 * t26338;
    let t97529 = t22751 * t28213;
    let t97537 = t22751 * t28210;
    let t97548 = t6883 * t28233;
    let t97552 = t90702 + F::cast_from(0.38381794893125283518e-1_f64) * t97537 - t3882 * t28187 - t22656 * t6461 + t19804 * t2006 * t568 + t6361 * t6955 * t568 + F::cast_from(0.16449340668482264365e-1_f64) * t81284 + t90708 + t1323 * t28107 * t568 - F::cast_from(0.38381794893125283518e-1_f64) * t97548 - F::new(2.0) * t16439 * t7750;
    (t97527, t97529, t97552)
}
