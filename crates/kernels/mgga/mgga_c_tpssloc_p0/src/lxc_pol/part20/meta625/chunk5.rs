//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2254/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2254<F: Float>(t13225: F, t9638: F, t13177: F, t13222: F, t13231: F, t13242: F, t13254: F, t13262: F, t13263: F, t1484: F, t1495: F, t210: F, t2643: F, t2686: F, t40971: F, t41161: F, t4180: F, t4181: F, t46644: F, t46675: F, t46677: F, t46679: F, t46686: F, t46692: F, t46693: F, t820: F, t829: F, t843: F, t9458: F, t9642: F, t9661: F) -> F {
    let t46698 = t9638 * t13225;
    let t46716 = -t13177 * t2686 / F::new(1024.0) + F::new(7.0) / F::new(1536.0) * t46675 + F::new(7.0) / F::new(768.0) * t46677 + F::new(35.0) / F::new(64.0) * t46679 + F::new(35.0) / F::new(128.0) * t843 * t40971 * t820 * t1484 * t9458 + F::new(7.0) / F::new(4.0) * t46686 + F::new(5.0) / F::new(4.0) * t41161 * t210 * t1495 * t9458 - t2643 * t46692 * t46693 * t829 / F::new(1024.0) - F::new(7.0) / F::new(192.0) * t46698 + t2643 * t13222 * t46644 * t829 / F::new(256.0) + t9642 * t13225 / F::new(128.0) - t13254 * t13231 / F::new(64.0) - F::new(3.0) / F::new(512.0) * t13262 * t4180 * t13242 * t13263 - t2643 * t4180 * t4181 * t9661 / F::new(3072.0);
    t46716
}
