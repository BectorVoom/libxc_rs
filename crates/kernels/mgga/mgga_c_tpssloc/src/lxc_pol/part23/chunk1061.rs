//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1061/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1061<F: Float>(t215: F, t39933: F, t535: F, t116: F, t557: F, t1314: F, t9534: F, t59: F, t9223: F, t120: F, t212: F, t22815: F, t67: F, t9580: F, t2566: F, t3732: F) -> (F, F, F, F, F, F, F, F) {
    let t40350 = 0.14979423868312757201e0 * t39933 * t535 * t215;
    let t40353 = t557 * t116;
    let t40369 = t9534 * t1314 * t116;
    let t40394 = t59 * t9223;
    let t40399 = t116 * t67 * t22815 * t120 * t212;
    let t40401 = 0.69444444444444444445e-4 * t40394 * t535 * t40399;
    let t40406 = t9580 * t1314;
    let t40409 = t2566 * t3732;
    (t40350, t40353, t40369, t40394, t40399, t40401, t40406, t40409)
}
