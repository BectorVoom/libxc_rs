//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta656<F: Float>(t1458: F, t2311: F, t1873: F, t22479: F, t7676: F, t7467: F, t9348: F, t45632: F, t12734: F, t2314: F, t26135: F, t12739: F) -> (F, F, F, F, F, F, F, F) {
        let (t90381, t90383, t90385, t90387, t90399, t90404, t90406, t90408) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2182::<F>(t1458, t2311, t1873, t22479, t7676, t7467, t9348, t45632, t12734, t2314, t26135, t12739);
    (t90381, t90383, t90385, t90387, t90399, t90404, t90406, t90408)
}
