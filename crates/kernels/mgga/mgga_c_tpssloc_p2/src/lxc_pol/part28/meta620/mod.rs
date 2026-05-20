//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1940;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta620<F: Float>(t1339: F, t57643: F, t6936: F, t22827: F, t550: F, t56805: F, t54165: F, t16060: F, t6944: F, t1354: F, t1827: F, t80991: F, t22765: F, t5289: F, t22764: F, t5234: F, t26298: F, t80958: F, t1307: F, t5287: F, t54068: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91268, t91272, t91276, t91279, t91281) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1940::<F>(t1339, t57643, t6936, t22827, t550, t56805, t54165, t16060, t6944, t1354, t1827, t80991);
        let (t91283, t91286, t91290, t91294, t91298) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1941::<F>(t22765, t5289, t22764, t5234, t1354, t26298, t80958, t1307, t1339, t22827, t5287, t54068, t550);
    (t91268, t91272, t91276, t91279, t91281, t91283, t91286, t91290, t91294, t91298)
}
