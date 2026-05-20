//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2247/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2247<F: Float>(t1496: F, t41083: F, t4257: F, t9601: F, t13193: F, t2697: F, t13204: F, t2563: F, t2379: F, t40959: F, t40962: F, t40966: F, t40982: F, t40984: F, t40988: F, t40990: F, t40998: F, t4119: F, t820: F, t843: F, t9607: F) -> F {
    let t46546 = t41083 * t1496;
    let t46549 = t9601 * t4257;
    let t46550 = F::new(595.0) / F::new(1152.0) * t46549;
    let t46551 = t2697 * t13193;
    let t46558 = t2563 * t13204;
    let t46560 = -F::new(35.0) / F::new(384.0) * t40959 + F::new(7.0) / F::new(384.0) * t40962 + F::new(595.0) / F::new(864.0) * t40966 - F::new(119.0) / F::new(1152.0) * t40982 + F::new(7.0) / F::new(1152.0) * t40984 + F::new(35.0) / F::new(192.0) * t40988 + F::new(595.0) / F::new(1152.0) * t40990 + F::new(455.0) / F::new(648.0) * t46546 - F::new(7.0) / F::new(16.0) * t40998 + t46550 - F::new(35.0) / F::new(192.0) * t46551 - F::new(15.0) / F::new(128.0) * t843 * t9607 * t820 * t4119 * t2379 + F::new(7.0) / F::new(48.0) * t46558;
    t46560
}
