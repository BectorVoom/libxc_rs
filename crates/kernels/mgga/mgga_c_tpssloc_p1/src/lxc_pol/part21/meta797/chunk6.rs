//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2771/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2771<F: Float>(t120: F, t16752: F, t16924: F, t9638: F, t17004: F, t2563: F, t12971: F, t13191: F, t13222: F, t13229: F, t13242: F, t13262: F, t13263: F, t13333: F, t16836: F, t16839: F, t16891: F, t16903: F, t16912: F, t17013: F, t17017: F, t20986: F, t232: F, t2643: F, t2645: F, t2679: F, t41467: F, t4178: F, t4180: F, t4181: F, t4248: F, t46558: F, t46573: F, t46577: F, t46628: F, t47307: F, t58246: F, t829: F, t9642: F) -> (F, F) {
    let t58495 = t120 * t16752;
    let t58504 = t9638 * t16924;
    let t58528 = t2563 * t17004;
    let t58540 = -t9642 * t17013 / F::new(1536.0) - t2643 * t4180 * t16839 * t2679 / F::new(3072.0) - t9642 * t17017 / F::new(1536.0) - t2643 * t4180 * t58495 * t829 / F::new(1536.0) - t2643 * t4180 * t16891 * t2679 / F::new(3072.0) - F::new(7.0) / F::new(288.0) * t58504 + t2643 * t2645 * t13242 * t16912 / F::new(192.0) + t2643 * t2645 * t4181 * t232 * t12971 / F::new(384.0) + t9642 * t16903 / F::new(384.0) + t16836 * t13333 / F::new(256.0) + t47307 * t4180 * t16839 * t58246 / F::new(128.0) - F::new(3.0) / F::new(256.0) * t13262 * t4180 * t16839 * t13263 + F::new(7.0) / F::new(72.0) * t46558 + F::new(7.0) / F::new(72.0) * t58528 - F::new(5.0) / F::new(32.0) * t46628 * t41467 * t4248 * t13191 - t4178 * t13222 * t20986 * t13229 / F::new(192.0) - F::new(119.0) / F::new(864.0) * t46573 + F::new(595.0) / F::new(1296.0) * t46577;
    (t58495, t58540)
}
