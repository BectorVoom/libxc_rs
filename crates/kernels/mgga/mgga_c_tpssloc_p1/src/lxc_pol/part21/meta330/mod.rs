//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta330<F: Float>(t12345: F, t1369: F, t241: F, t67: F, t6924: F, t3866: F, t3872: F, t3876: F, t1339: F, t2690: F, t1336: F) -> (F, F, F, F, F, F) {
        let (t12346, t12351, t12356, t12358, t12364, t12365) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1705::<F>(t12345, t1369, t241, t67, t6924, t3866, t3872, t3876, t1339, t2690, t1336);
    (t12346, t12351, t12356, t12358, t12364, t12365)
}
