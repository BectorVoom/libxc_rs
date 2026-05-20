//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2417;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta633<F: Float>(t2578: F, t41189: F, t9546: F, t9555: F, t2573: F, t41008: F, t2566: F, t2570: F, t9551: F, t2588: F, t40341: F, t207: F, t215: F, t39933: F, t40344: F, t795: F, t116: F, t786: F, t9534: F, t133: F, t6600: F, t776: F, t39568: F, t761: F, t2535: F, t9716: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41190, t41192, t41194, t41196, t41197, t41200, t41209) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2417::<F>(t2578, t41189, t9546, t9555, t2573, t41008, t2566, t2570, t9551, t2588, t40341, t207, t215, t39933);
        let (t41212, t41214, t41217, t41254, t41255) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2418::<F>(t207, t40344, t795, t116, t786, t9534, t133, t6600, t776, t39568, t761, t2535, t9716);
    (t41190, t41192, t41194, t41196, t41197, t41200, t41209, t41212, t41214, t41217, t41254, t41255)
}
