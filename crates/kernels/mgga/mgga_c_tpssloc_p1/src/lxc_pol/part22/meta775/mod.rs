//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta775 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta775<F: Float>(t39365: F, t56168: F, t54380: F, t54382: F, t39374: F, t54389: F, t56185: F, t54392: F, t15883: F, t19577: F, t19596: F, t19631: F, t3918: F, t39400: F, t39408: F, t39411: F, t39463: F, t39468: F, t5126: F, t5127: F, t6347: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t74040, t74041, t74042, t74043, t74044, t74046, t74056, t74057, t74058) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2650::<F>(t39365, t56168, t54380, t54382, t39374, t54389, t56185, t54392, t15883, t19577, t19596, t19631, t3918, t39400, t39408, t39411, t39463, t39468, t5126, t5127, t6347);
    (t74040, t74041, t74042, t74043, t74044, t74046, t74056, t74057, t74058)
}
