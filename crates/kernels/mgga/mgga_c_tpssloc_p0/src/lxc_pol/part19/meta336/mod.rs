//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1200;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta336<F: Float>(t39494: F, t761: F, t152: F, t185: F, t39097: F, t153: F, t157: F, t39842: F, t10140: F, t10143: F, t2374: F, t39354: F, t193: F, t202: F, t2522: F, t39529: F, t40760: F, t40762: F, t40764: F, t40766: F, t40768: F, t40769: F, t40772: F, t40777: F, t776: F) -> (F, F, F, F, F) {
        let (t40779, t40782, t40784, t40785, t40790) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1200::<F>(t39494, t761, t152, t185, t39097, t153, t157, t39842, t10140, t10143, t2374, t39354);
        let t40791 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1201::<F>(t193, t202, t2522, t39529, t40760, t40762, t40764, t40766, t40768, t40769, t40772, t40777, t40779, t40782, t40784, t40785, t40790, t776);
    (t40779, t40782, t40784, t40790, t40791)
}
