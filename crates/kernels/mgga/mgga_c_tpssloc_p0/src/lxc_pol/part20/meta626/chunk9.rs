//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2265/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2265<F: Float>(t2635: F, t46881: F, t13337: F, t838: F, t2693: F, t4163: F, t13080: F, t13084: F, t13223: F, t13251: F, t13254: F, t13262: F, t13350: F, t1495: F, t210: F, t2553: F, t2571: F, t2643: F, t2645: F, t4158: F, t4248: F, t46870: F, t46875: F, t46876: F, t46878: F, t9516: F, t9642: F, t9647: F, t9649: F, t9976: F) -> F {
    let t46882 = t46881 * t2635;
    let t46884 = t13337 * t838;
    let t46886 = t4163 * t2693;
    let t46887 = F::new(119.0) / F::new(4608.0) * t46886;
    let t46910 = -F::new(7.0) / F::new(384.0) * t46870 + t46875 + F::new(595.0) / F::new(10368.0) * t46876 + t46878 * t2635 / F::new(512.0) - F::new(7.0) / F::new(768.0) * t46882 - F::new(7.0) / F::new(1536.0) * t46884 + t46887 - t13254 * t13084 / F::new(128.0) - F::new(5.0) / F::new(256.0) * t9642 * t13080 - F::new(5.0) / F::new(256.0) * t13251 * t9649 + t13262 * t2645 * t4248 * t9976 / F::new(128.0) - F::new(5.0) / F::new(256.0) * t2643 * t13350 * t13223 * t9647 + F::new(3.0) / F::new(16.0) * t2571 * t210 * t4158 * t2553 + t2571 * t210 * t1495 * t9516 / F::new(16.0);
    t46910
}
