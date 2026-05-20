//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta538<F: Float>(t39537: F, t761: F, t2531: F, t9494: F, t39344: F, t39362: F, t2427: F, t9868: F, t2751: F, t39494: F, t153: F, t157: F, t39842: F) -> (F, F, F, F, F, F, F, F) {
        let (t40760, t40761, t40764, t40766, t40767, t40772, t40779, t40784) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2079::<F>(t39537, t761, t2531, t9494, t39344, t39362, t2427, t9868, t2751, t39494, t153, t157, t39842);
    (t40760, t40761, t40764, t40766, t40767, t40772, t40779, t40784)
}
