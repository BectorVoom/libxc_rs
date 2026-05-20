//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta639<F: Float>(t14025: F, t23537: F, t13970: F, t23541: F, t4616: F, t6764: F, t23544: F, t4571: F, t23482: F, t25682: F, t25588: F, t344: F, t6740: F) -> (F, F, F, F, F, F) {
        let (t88249, t88251, t88277, t88281, t88286, t88290) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2049::<F>(t14025, t23537, t13970, t23541, t4616, t6764, t23544, t4571, t23482, t25682, t25588, t344, t6740);
    (t88249, t88251, t88277, t88281, t88286, t88290)
}
