//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2537;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta705<F: Float>(t3067: F, t353: F, t373: F, t383: F, t1021: F, t820: F, t10482: F, t1615: F, t1041: F, t13969: F, t14142: F, t14179: F, t10375: F, t1612: F, t1539: F, t248: F, t42749: F, t10661: F, t1556: F, t14363: F, t300: F, t14419: F, t923: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48607, t48611, t48612, t48626, t48629) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2537::<F>(t3067, t353, t373, t383, t1021, t820, t10482, t1615, t1041, t13969, t14142, t14179);
        let (t48670, t48674, t48763, t48766, t48771) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2538::<F>(t10375, t1612, t1041, t1539, t248, t42749, t10661, t1556, t14363, t300, t14419, t923);
    (t48607, t48611, t48612, t48626, t48629, t48670, t48674, t48763, t48766, t48771)
}
