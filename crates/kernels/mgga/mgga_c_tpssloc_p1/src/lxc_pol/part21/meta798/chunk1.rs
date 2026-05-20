//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2773/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2773<F: Float>(t13257: F, t4166: F, t4184: F, t10007: F, t13222: F, t13251: F, t13262: F, t13263: F, t13312: F, t13350: F, t16891: F, t16944: F, t16949: F, t2633: F, t2643: F, t2645: F, t2647: F, t41063: F, t4178: F, t4180: F, t46597: F, t46661: F, t46663: F, t46668: F, t46675: F, t46677: F, t46679: F, t46686: F, t47017: F, t5591: F, t5593: F, t58495: F, t829: F) -> F {
    let t58616 = t4166 * t13257 * t4184;
    let t58628 = -F::new(7.0) / F::new(576.0) * t46661 - F::new(35.0) / F::new(288.0) * t46663 + F::new(7.0) / F::new(1152.0) * t46668 + F::new(7.0) / F::new(2304.0) * t46675 + F::new(7.0) / F::new(1152.0) * t46677 + F::new(35.0) / F::new(96.0) * t46679 + t2643 * t13222 * t47017 * t5591 / F::new(192.0) - F::new(5.0) / F::new(192.0) * t2643 * t13350 * t16944 * t829 - F::new(5.0) / F::new(384.0) * t2643 * t13350 * t16949 * t829 + F::new(7.0) / F::new(6.0) * t46686 - t13251 * t13312 / F::new(768.0) - t13262 * t4180 * t16891 * t13263 / F::new(512.0) + t2643 * t2645 * t58495 * t2647 / F::new(384.0) + t2643 * t2645 * t16891 * t10007 / F::new(768.0) - F::new(7.0) / F::new(576.0) * t58616 + t41063 * t5593 / F::new(384.0) + t4178 * t4180 * t16891 * t2633 / F::new(512.0) + t2643 * t2645 * t46597 * t5591 / F::new(384.0);
    t58628
}
