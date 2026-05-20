//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta456<F: Float>(t1307: F, t1388: F, t118: F, t1787: F) -> (F, F) {
        let (t15904, t15908) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2016::<F>(t1307, t1388, t118, t1787);
    (t15904, t15908)
}
