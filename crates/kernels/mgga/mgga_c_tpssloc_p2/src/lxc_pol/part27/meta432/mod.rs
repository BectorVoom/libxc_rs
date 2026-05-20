//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1752;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta432<F: Float>(t22751: F, t6970: F, t3853: F, t6945: F, t3777: F, t6944: F, t1354: F, t3787: F, t59: F, t240: F, t1336: F, t3795: F, t6943: F, t835: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22752, t22753, t22754, t22756, t22757, t22759, t22760, t22762) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1752::<F>(t22751, t6970, t3853, t6945, t3777, t6944, t1354, t3787, t59, t240, t1336, t3795);
        let (t22764, t22765) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1753::<F>(t6943, t835, t1336);
    (t22752, t22753, t22754, t22756, t22757, t22759, t22760, t22762, t22764, t22765)
}
