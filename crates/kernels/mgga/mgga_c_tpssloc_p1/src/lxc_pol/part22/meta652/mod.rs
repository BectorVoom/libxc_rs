//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta652<F: Float>(t16841: F, t46741: F, t17017: F, t9638: F, t41107: F, t5593: F, t16914: F, t17009: F, t41115: F, t13258: F, t16932: F, t16937: F) -> (F, F, F, F, F, F, F, F) {
        let (t58353, t58363, t58373, t58379, t58381, t58421, t58425, t58427) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2194::<F>(t16841, t46741, t17017, t9638, t41107, t5593, t16914, t17009, t41115, t13258, t16932, t16937);
    (t58353, t58363, t58373, t58379, t58381, t58421, t58425, t58427)
}
