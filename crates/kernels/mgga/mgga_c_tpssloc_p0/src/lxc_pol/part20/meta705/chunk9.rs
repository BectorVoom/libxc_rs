//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2687/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2687<F: Float>(t16081: F, t16086: F, t12214: F, t67: F, t792: F, t16095: F, t3734: F, t686: F, t133: F, t1799: F, t40369: F, t6600: F) -> (F, F, F) {
    let t54711 = t16081 * t16086;
    let t54718 = t792 * t12214 * t67;
    let t54721 = t54718 * t686 * t16095 * t3734;
    let t54725 = t40369 * t133 * t6600 * t1799;
    (t54711, t54721, t54725)
}
