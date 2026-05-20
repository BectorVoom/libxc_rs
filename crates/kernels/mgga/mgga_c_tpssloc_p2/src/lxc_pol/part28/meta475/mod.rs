//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1686;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1687;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta475<F: Float>(t25338: F, t6552: F, t4119: F, t6554: F, t6553: F, t23204: F, t7479: F, t23164: F, t1530: F, t776: F, t22960: F, t10143: F, t25: F, t868: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25339, t25341, t25342, t25343, t25345, t25346, t25365) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1686::<F>(t25338, t6552, t4119, t6554, t6553, t23204, t7479, t23164, t1530, t776);
        let (t25366, t25373) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1687::<F>(t22960, t25365, t10143, t25);
        let t25374 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1688::<F>(t1530, t868);
    (t25339, t25341, t25342, t25343, t25345, t25346, t25365, t25366, t25373, t25374)
}
