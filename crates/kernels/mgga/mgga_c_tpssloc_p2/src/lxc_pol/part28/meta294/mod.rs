//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta294<F: Float>(t10228: F, t973: F, t135: F, t2978: F, t2981: F, t4509: F, t984: F, t2770: F, t343: F, t2244: F, t2987: F, t3008: F) -> (F, F, F, F, F, F, F) {
        let (t10229, t10231, t10233, t10235, t10236, t10237, t10241) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1204::<F>(t10228, t973, t135, t2978, t2981, t4509, t984, t2770, t343, t2244, t2987, t3008);
    (t10229, t10231, t10233, t10235, t10236, t10237, t10241)
}
