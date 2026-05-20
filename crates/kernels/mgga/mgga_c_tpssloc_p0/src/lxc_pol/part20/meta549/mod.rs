//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2093;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta549<F: Float>(t13012: F, t9566: F, t207: F, t215: F, t39933: F, t40344: F, t795: F, t116: F, t786: F, t9534: F, t133: F, t6600: F, t776: F, t2639: F, t9960: F, t2427: F, t9909: F, t39568: F, t761: F, t2535: F, t9716: F, t39382: F, t2531: F, t9713: F, t39302: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41205, t41209, t41212, t41214, t41217) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2093::<F>(t13012, t9566, t207, t215, t39933, t40344, t795, t116, t786, t9534, t133, t6600, t776);
        let (t41237, t41251, t41254, t41255, t41258, t41259, t41262) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2094::<F>(t2639, t9960, t2427, t9909, t39568, t761, t2535, t9716, t39382, t2531, t9713, t39302);
    (t41205, t41209, t41212, t41214, t41217, t41237, t41251, t41254, t41255, t41258, t41259, t41262)
}
