//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta386<F: Float>(t12344: F, t5234: F, t1831: F, t40059: F, t12282: F, t12290: F, t12384: F, t1827: F, t40123: F, t1788: F, t9212: F, t9214: F) -> (F, F, F, F, F, F, F, F) {
        let (t53880, t53901, t53945, t54020, t54042, t54151, t54312, t54314) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1190::<F>(t12344, t5234, t1831, t40059, t12282, t12290, t12384, t1827, t40123, t1788, t9212, t9214);
    (t53880, t53901, t53945, t54020, t54042, t54151, t54312, t54314)
}
