//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2293/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2293<F: Float>(t2627: F, t4265: F, t226: F, t40931: F, t68: F, t13377: F, t814: F, t10073: F, t10081: F, t13176: F, t13380: F, t13397: F, t13416: F, t13423: F, t2617: F, t2633: F, t2736: F, t4166: F, t4281: F, t4282: F, t4288: F, t47308: F, t812: F, t829: F, t9612: F, t9976: F, t9981: F) -> F {
    let t47374 = t2627 * t4265;
    let t47386 = t226 * t68 * t40931;
    let t47395 = t814 * t13377;
    let t47399 = F::new(18.0) * t13380 * t2633 * t4281 - F::new(36.0) * t13397 * t4282 * t9976 + F::new(6.0) * t13416 * t812 * t9981 + F::new(6.0) * t2633 * t47374 * t812 + F::new(24.0) * t4282 * t47308 * t47386 - F::new(3.0) * t47395 * t812 * t829 - F::new(3.0) * t10073 * t4166 - F::new(6.0) * t10081 * t4166 - F::new(3.0) * t13176 * t2736 - F::new(3.0) * t13423 * t2617 - F::new(3.0) * t4288 * t9612;
    t47399
}
