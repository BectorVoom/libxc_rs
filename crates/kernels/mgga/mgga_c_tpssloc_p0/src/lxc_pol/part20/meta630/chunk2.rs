//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2286/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2286<F: Float>(t12971: F, t13141: F, t13151: F, t13160: F, t13161: F, t13164: F, t13167: F, t1504: F, t16729: F, t1891: F, t232: F, t2379: F, t2553: F, t2667: F, t4119: F, t4225: F, t4227: F, t47213: F, t68: F, t776: F, t822: F, t825: F, t845: F, t9947: F, t9951: F) -> F {
    let t47215 = (-F::new(36.0) * t12971 * t4225 * t776 * t845 + F::new(180.0) * t1891 * t2379 * t4119 * t4225 - F::new(36.0) * t13160 * t2553 * t4225 - F::new(36.0) * t2667 * t4227 * t68 + F::new(9.0) * t13141 * t825 - F::new(72.0) * t13151 * t13161 - F::new(36.0) * t13151 * t13164 + F::new(9.0) * t13167 * t822 + F::new(60.0) * t1504 * t9947 - F::new(36.0) * t16729 * t9951 + t47213) * t232;
    t47215
}
