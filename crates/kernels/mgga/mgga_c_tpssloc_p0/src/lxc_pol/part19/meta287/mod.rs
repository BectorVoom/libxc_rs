//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta287<F: Float>(t12283: F, t3809: F, t3777: F, t3789: F, t12248: F, t236: F, t240: F, t1336: F, t12251: F, t1343: F, t820: F, t12255: F) -> (F, F, F, F, F, F, F) {
        let (t12284, t12286, t12289, t12290, t12291, t12293, t12297) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1055::<F>(t12283, t3809, t3777, t3789, t12248, t236, t240, t1336, t12251, t1343, t820, t12255);
    (t12284, t12286, t12289, t12290, t12291, t12293, t12297)
}
