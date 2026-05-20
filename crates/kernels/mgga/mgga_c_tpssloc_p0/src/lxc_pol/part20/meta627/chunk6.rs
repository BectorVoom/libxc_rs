//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2272/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2272<F: Float>(t13005: F, t13184: F, t13196: F, t13203: F, t13222: F, t13242: F, t13350: F, t210: F, t221: F, t2571: F, t2643: F, t2645: F, t2649: F, t41014: F, t41116: F, t4178: F, t4180: F, t4181: F, t4182: F, t4248: F, t46644: F, t46839: F, t47027: F, t47037: F, t47039: F, t47044: F, t47047: F, t47049: F, t776: F, t829: F, t9632: F, t9981: F) -> F {
    let t47071 = F::new(7.0) / F::new(192.0) * t47027 + t4178 * t4180 * t4181 * t41014 / F::new(1536.0) + t4178 * t4180 * t13242 * t9632 / F::new(512.0) + F::new(35.0) / F::new(192.0) * t47037 + F::new(15.0) / F::new(128.0) * t2643 * t47039 * t13184 * t829 + t47044 * t2649 / F::new(128.0) - F::new(595.0) / F::new(10368.0) * t47047 - F::new(7.0) / F::new(8.0) * t47049 + F::new(3.0) / F::new(16.0) * t2571 * t210 * t13203 * t776 - t4178 * t13222 * t46644 * t4182 / F::new(128.0) - F::new(3.0) / F::new(4.0) * t13005 * t221 * t46839 - F::new(5.0) / F::new(256.0) * t2643 * t13350 * t13196 * t829 - t4178 * t2645 * t4248 * t9981 / F::new(128.0) + F::new(119.0) / F::new(576.0) * t41116;
    t47071
}
