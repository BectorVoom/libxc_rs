//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta486<F: Float>(t5234: F, t6951: F, t1369: F, t1831: F, t22788: F, t5314: F, t6952: F, t1811: F, t22797: F, t22804: F, t7709: F, t1361: F, t1799: F, t22690: F) -> (F, F, F, F, F, F, F) {
        let (t26257, t26258, t26260, t26262, t26266, t26268, t26271) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1700::<F>(t5234, t6951, t1369, t1831, t22788, t5314, t6952, t1811, t22797, t22804, t7709, t1361, t1799, t22690);
    (t26257, t26258, t26260, t26262, t26266, t26268, t26271)
}
