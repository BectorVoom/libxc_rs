//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2795/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2795<F: Float>(t12971: F, t13141: F, t13151: F, t13157: F, t13161: F, t13167: F, t1504: F, t1506: F, t16662: F, t16729: F, t16736: F, t16740: F, t16745: F, t16746: F, t225: F, t230: F, t2379: F, t2553: F, t2672: F, t4225: F, t4226: F, t5527: F, t5601: F, t58963: F, t58964: F, t58966: F, t58967: F, t58970: F, t58981: F, t59010: F, t59050: F, t6589: F, t776: F, t845: F) -> F {
    let t59072 = F::new(6.0) * t13141 * t1506 - F::new(48.0) * t16729 * t13161 + F::new(6.0) * t1504 * t13167 - F::new(24.0) * t4225 * t845 * t16662 * t776 - F::new(12.0) * t4225 * t16745 * t2553 - F::new(24.0) * t13151 * t16746 - (t58963 + t58964 + t58966 + t58967 + t58970 + t58981 + t59010 + t59050) * t225 * t230 + F::new(60.0) * t4225 * t16736 * t2553 - F::new(24.0) * t4225 * t4226 * t12971 - F::new(12.0) * t5601 * t2672 - F::new(48.0) * t13151 * t16740 + F::new(120.0) * t16729 * t13157 - F::new(360.0) * t4225 * t6589 * t5527 * t2379;
    t59072
}
