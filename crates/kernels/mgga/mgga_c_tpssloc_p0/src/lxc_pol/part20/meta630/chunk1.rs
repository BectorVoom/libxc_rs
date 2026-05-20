//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2285/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2285<F: Float>(t13151: F, t13156: F, t13157: F, t1484: F, t1504: F, t1506: F, t225: F, t228: F, t230: F, t2667: F, t2672: F, t2675: F, t4219: F, t4225: F, t4226: F, t4230: F, t46426: F, t47138: F, t47139: F, t47141: F, t47142: F, t47145: F, t47146: F, t47148: F, t47187: F, t6589: F, t824: F, t9458: F, t9516: F, t9616: F, t9938: F, t9954: F) -> F {
    let t47213 = F::new(3.0) * t228 * t824 * t46426 + F::new(9.0) * t4219 * t2675 - (t47138 + t47139 + t47141 + t47142 + t47145 + t47146 + t47148 + t47187) * t225 * t230 - F::new(12.0) * t4225 * t4226 * t9516 + F::new(3.0) * t1504 * t9954 + F::new(3.0) * t9938 * t1506 - F::new(360.0) * t4225 * t6589 * t1484 * t9458 + F::new(180.0) * t4225 * t13156 * t9616 - F::new(36.0) * t4219 * t2672 + F::new(9.0) * t2667 * t4230 + F::new(180.0) * t13151 * t13157;
    t47213
}
