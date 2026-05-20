//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta618<F: Float>(t23384: F, t23582: F, t23333: F, t82431: F, t23323: F, t6683: F, t23357: F, t6680: F, t23494: F, t381: F, t23403: F, t23589: F) -> (F, F, F, F, F, F, F) {
        let (t83318, t83329, t83342, t83344, t83352, t83358, t83364) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2096::<F>(t23384, t23582, t23333, t82431, t23323, t6683, t23357, t6680, t23494, t381, t23403, t23589);
    (t83318, t83329, t83342, t83344, t83352, t83358, t83364)
}
