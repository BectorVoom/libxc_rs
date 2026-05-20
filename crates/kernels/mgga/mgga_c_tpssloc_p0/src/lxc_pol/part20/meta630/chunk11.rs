//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2295/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2295<F: Float>(t10055: F, t13380: F, t13384: F, t13385: F, t13407: F, t13414: F, t13434: F, t13453: F, t25236: F, t2613: F, t2617: F, t2679: F, t4166: F, t4281: F, t4286: F, t4291: F, t4298: F, t47425: F, t829: F, t9612: F, t9632: F) -> F {
    let t47507 = F::new(6.0) * t13380 * t4281 * t9632 + F::new(6.0) * t13384 * t4281 * t9632 - F::new(3.0) * t25236 * t2679 * t4291 - F::new(3.0) * t4291 * t47425 * t829 + F::new(6.0) * t10055 * t4166 + F::new(12.0) * t13385 * t13453 - F::new(6.0) * t13407 * t2617 - F::new(3.0) * t13414 * t2617 - F::new(6.0) * t13434 * t2617 + F::new(3.0) * t2613 * t4298 - F::new(3.0) * t4286 * t9612;
    t47507
}
