//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1573;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1574;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta283<F: Float>(t10236: F, t2244: F, t2987: F, t3008: F, t2250: F, t2989: F, t2775: F, t343: F, t3014: F, t2262: F, t972: F, t2960: F, t2971: F, t2970: F, t2995: F, t973: F, t2769: F, t40: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10237, t10241, t10245, t10254) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1573::<F>(t10236, t2244, t2987, t3008, t2250, t2989, t2775, t343);
        let (t10255, t10259, t10263) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1574::<F>(t10254, t2244, t2987, t3014, t2262, t972);
        let (t10267, t10274, t10276, t10277) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1575::<F>(t2960, t2971, t2970, t2995, t973, t2769, t40);
    (t10237, t10241, t10245, t10254, t10255, t10259, t10263, t10267, t10274, t10276, t10277)
}
