//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta303<F: Float>(t698: F, t976: F, t979: F, t973: F, t135: F, t2978: F, t4509: F, t984: F, t2770: F, t343: F, t2775: F, t2769: F, t40: F) -> (F, F, F, F, F, F, F) {
        let (t10224, t10226, t10231, t10235, t10236, t10254, t10276) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1328::<F>(t698, t976, t979, t973, t135, t2978, t4509, t984, t2770, t343, t2775, t2769, t40);
    (t10224, t10226, t10231, t10235, t10236, t10254, t10276)
}
