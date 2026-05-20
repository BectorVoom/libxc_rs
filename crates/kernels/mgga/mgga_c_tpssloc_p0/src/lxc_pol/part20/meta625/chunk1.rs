//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2250/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2250<F: Float>(t10003: F, t13222: F, t13228: F, t13229: F, t13251: F, t13300: F, t13353: F, t16935: F, t2633: F, t2643: F, t2645: F, t41025: F, t41031: F, t41467: F, t4178: F, t4180: F, t4182: F, t4248: F, t46595: F, t46597: F, t46606: F, t46611: F, t46616: F, t46618: F, t46628: F, t829: F, t9616: F, t9642: F) -> F {
    let t46637 = -F::new(7.0) / F::new(384.0) * t46595 - t2643 * t4180 * t46597 * t829 / F::new(1024.0) - t4178 * t13222 * t16935 * t13229 / F::new(64.0) - t4178 * t13222 * t13228 * t46606 / F::new(128.0) + F::new(7.0) / F::new(96.0) * t46611 - F::new(5.0) / F::new(128.0) * t9642 * t13353 - F::new(7.0) / F::new(384.0) * t41025 - F::new(7.0) / F::new(192.0) * t46616 - F::new(7.0) / F::new(384.0) * t46618 + F::new(7.0) / F::new(1536.0) * t41031 + t13251 * t10003 / F::new(256.0) + t4178 * t4180 * t46597 * t4182 / F::new(512.0) - F::new(15.0) / F::new(128.0) * t46628 * t41467 * t4248 * t9616 - t4178 * t2645 * t13300 * t2633 / F::new(128.0);
    t46637
}
