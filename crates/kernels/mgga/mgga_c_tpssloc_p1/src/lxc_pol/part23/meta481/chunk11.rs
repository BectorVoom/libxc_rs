//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1450/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1450<F: Float>(t1227: F, t15453: F, t1730: F, t22174: F, t4582: F, t488: F, t6232: F, t65552: F, t65558: F, t65581: F, t65706: F, t72273: F, t72285: F, t72287: F, t72289: F, t72293: F, t72297: F, t72302: F, t77606: F) -> F {
    let t78734 = -F::new(5.0) / F::new(864.0) * t1227 * t4582 * t15453 * t77606 + t65552 / F::new(1728.0) + t65706 * t6232 / F::new(48.0) - t72273 / F::new(1728.0) - t65558 / F::new(1152.0) - t72285 / F::new(288.0) + t72287 / F::new(192.0) + t72289 / F::new(108.0) + t72293 / F::new(1152.0) - t72297 / F::new(192.0) - F::new(19.0) / F::new(324.0) * t72302 - F::new(209.0) / F::new(648.0) * t1730 * t22174 * t488 - t65581 / F::new(2304.0);
    t78734
}
