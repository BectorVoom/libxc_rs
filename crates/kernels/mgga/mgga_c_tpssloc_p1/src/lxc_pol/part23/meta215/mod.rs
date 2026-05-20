//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta215<F: Float>(t1553: F, t2403: F, t1543: F, t2791: F, t1597: F, t4509: F, t10189: F, t344: F, t134: F, t2978: F, t10213: F, t60: F) -> (F, F, F, F, F, F, F) {
        let (t13642, t13727, t13769, t13779, t13783, t13784, t13797) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk860::<F>(t1553, t2403, t1543, t2791, t1597, t4509, t10189, t344, t134, t2978, t10213, t60);
    (t13642, t13727, t13769, t13779, t13783, t13784, t13797)
}
