//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1141;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta348<F: Float>(t59: F, t598: F, t535: F, t795: F, t215: F, t39933: F, t116: F, t557: F, t1314: F, t9534: F, t9223: F, t120: F, t212: F, t22815: F, t67: F, t9580: F, t2566: F, t3732: F, t12214: F, t792: F, t2229: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40344, t40347, t40350, t40353, t40369, t40394, t40399) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1141::<F>(t59, t598, t535, t795, t215, t39933, t116, t557, t1314, t9534, t9223, t120, t212, t22815, t67);
        let (t40401, t40406, t40409, t40412, t40419) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1142::<F>(t40394, t40399, t535, t1314, t9580, t2566, t3732, t12214, t792, t2229, t59, t60);
    (t40344, t40347, t40350, t40353, t40369, t40394, t40399, t40401, t40406, t40409, t40412, t40419)
}
