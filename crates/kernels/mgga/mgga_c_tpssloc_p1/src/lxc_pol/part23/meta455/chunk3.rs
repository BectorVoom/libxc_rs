//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1316/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1316<F: Float>(t119: F, t13251: F, t16836: F, t16839: F, t20885: F, t20974: F, t20978: F, t20986: F, t20988: F, t210: F, t2571: F, t2643: F, t2645: F, t2701: F, t41161: F, t4178: F, t4180: F, t46546: F, t5591: F, t58421: F, t67620: F, t67660: F, t67675: F, t76056: F, t76063: F, t820: F, t843: F) -> F {
    let t76167 = t16836 * t20988 / F::new(128.0) + F::new(455.0) / F::new(162.0) * t46546 + F::new(119.0) / F::new(288.0) * t58421 + F::new(3.0) / F::new(256.0) * t4178 * t4180 * t16839 * t20986 - F::new(5.0) / F::new(64.0) * t13251 * t20974 + t13251 * t20978 / F::new(64.0) + t2643 * t2645 * t16839 * t20885 / F::new(128.0) + t2643 * t2645 * t67620 * t5591 / F::new(192.0) + F::new(5.0) / F::new(4.0) * t41161 * t210 * t119 * t76056 + F::new(3.0) / F::new(16.0) * t2571 * t210 * t119 * t76063 + F::new(7.0) / F::new(192.0) * t67660 - F::new(35.0) / F::new(96.0) * t67675 + F::new(5.0) / F::new(256.0) * t843 * t2701 * t820 * t76063;
    t76167
}
