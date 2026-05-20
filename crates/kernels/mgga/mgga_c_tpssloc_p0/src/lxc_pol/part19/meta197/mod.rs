//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk863;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta197<F: Float>(t10250: F, t4518: F, t2775: F, t343: F, t2244: F, t2988: F, t2987: F, t3014: F, t2990: F, t2262: F, t972: F, t10186: F, t10192: F, t10196: F, t10200: F, t10204: F, t10209: F, t10219: F, t10226: F, t10229: F, t10233: F, t10238: F, t10242: F, t10246: F, t2960: F, t2982: F, t2986: F, t2991: F, t973: F, t980: F) -> (F, F, F, F, F, F, F, F) {
        let (t10251, t10254, t10255, t10256, t10259, t10260, t10263) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk863::<F>(t10250, t4518, t2775, t343, t2244, t2988, t2987, t3014, t2990, t2262, t972);
        let t10266 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk864::<F>(t10186, t10192, t10196, t10200, t10204, t10209, t10219, t10226, t10229, t10233, t10238, t10242, t10246, t10251, t10256, t10260, t10263, t2960, t2982, t2986, t2991, t973, t980);
    (t10251, t10254, t10255, t10256, t10259, t10260, t10263, t10266)
}
