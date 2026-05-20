//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2248/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2248<F: Float>(t4119: F, t828: F, t46528: F, t842: F, t4261: F, t9601: F, t1516: F, t40965: F, t13347: F, t2697: F, t119: F, t13248: F, t13254: F, t13350: F, t13365: F, t210: F, t2623: F, t2643: F, t2647: F, t2703: F, t40992: F, t41009: F, t41012: F, t4172: F, t46426: F, t787: F, t849: F, t9609: F, t9990: F) -> F {
    let t46565 = t4119 * t828;
    let t46570 = t46528 * t842;
    let t46573 = t9601 * t4261;
    let t46574 = F::new(119.0) / F::new(1152.0) * t46573;
    let t46577 = t40965 * t1516;
    let t46587 = t2697 * t13347;
    let t46593 = -t787 * t210 * t119 * t46426 / F::new(48.0) - F::new(5.0) / F::new(128.0) * t2643 * t13350 * t46565 * t2647 - t46570 * t849 / F::new(256.0) - t46574 - F::new(5.0) / F::new(128.0) * t4172 * t9609 + F::new(595.0) / F::new(2592.0) * t46577 + F::new(5.0) / F::new(256.0) * t13365 * t2703 - t40992 * t1516 / F::new(768.0) - t9990 * t4261 / F::new(256.0) - t2623 * t13347 / F::new(256.0) + F::new(7.0) / F::new(384.0) * t46587 + F::new(35.0) / F::new(24.0) * t41009 + F::new(7.0) / F::new(12.0) * t41012 + t13254 * t13248 / F::new(512.0);
    t46593
}
