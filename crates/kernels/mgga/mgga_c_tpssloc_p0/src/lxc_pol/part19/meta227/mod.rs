//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta227<F: Float>(t10956: F, t354: F, t1009: F, t3020: F, t1011: F, t1019: F, t1040: F, t3077: F, t2775: F, t283: F, t61: F, t10305: F, t248: F) -> (F, F, F, F, F, F, F, F) {
        let (t10957, t10960, t10961, t10962, t10965, t10969, t10970, t10972) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk933::<F>(t10956, t354, t1009, t3020, t1011, t1019, t1040, t3077, t2775, t283, t61, t10305, t248);
    (t10957, t10960, t10961, t10962, t10965, t10969, t10970, t10972)
}
