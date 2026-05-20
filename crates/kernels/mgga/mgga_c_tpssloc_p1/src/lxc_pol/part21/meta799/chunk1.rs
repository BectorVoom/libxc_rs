//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2780/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2780<F: Float>(t119: F, t12971: F, t13222: F, t13229: F, t13254: F, t13262: F, t13347: F, t13365: F, t1484: F, t1516: F, t16901: F, t16932: F, t16937: F, t16946: F, t210: F, t2553: F, t2623: F, t2643: F, t2645: F, t2684: F, t2701: F, t4172: F, t4191: F, t4261: F, t46570: F, t47037: F, t47044: F, t5527: F, t58139: F, t58845: F, t58847: F, t58853: F, t58859: F, t58873: F, t58885: F, t787: F, t820: F, t843: F, t9607: F) -> F {
    let t58887 = -t46570 * t1516 / F::new(384.0) - t13365 * t4261 / F::new(192.0) - t4172 * t13347 / F::new(384.0) + F::new(7.0) / F::new(576.0) * t58845 + F::new(7.0) / F::new(288.0) * t58847 - t787 * t210 * t119 * t58139 / F::new(48.0) + t13262 * t13222 * t58853 * t13229 / F::new(64.0) + F::new(35.0) / F::new(288.0) * t47037 + F::new(7.0) / F::new(2304.0) * t58859 + F::new(5.0) / F::new(192.0) * t2623 * t16946 + F::new(5.0) / F::new(384.0) * t843 * t2701 * t820 * t1484 * t12971 - F::new(5.0) / F::new(128.0) * t843 * t9607 * t820 * t5527 * t2553 - F::new(7.0) / F::new(288.0) * t58873 + t47044 * t4191 / F::new(192.0) + t2643 * t2645 * t16901 * t2684 / F::new(768.0) - t13254 * t16932 / F::new(192.0) + t13254 * t16937 / F::new(384.0) - F::new(7.0) / F::new(576.0) * t58885;
    t58887
}
