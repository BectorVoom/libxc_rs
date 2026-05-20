//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta511<F: Float>(t1955: F, t4693: F, t3174: F, t2775: F, t387: F, t3961: F, t23329: F, t221: F, t4509: F, t1926: F) -> (F, F, F, F, F, F) {
        let (t25420, t25423, t25424, t25425, t25428, t25429) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1915::<F>(t1955, t4693, t3174, t2775, t387, t3961, t23329, t221, t4509, t1926);
    (t25420, t25423, t25424, t25425, t25428, t25429)
}
