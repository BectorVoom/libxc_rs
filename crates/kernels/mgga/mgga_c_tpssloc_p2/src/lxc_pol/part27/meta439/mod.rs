//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta439<F: Float>(t22802: F, t22869: F, t553: F, t1338: F, t6955: F, t1352: F, t3851: F, t6987: F, t3856: F, t1372: F, t552: F, t1307: F) -> (F, F, F, F, F, F, F, F) {
        let (t22870, t22871, t22873, t22874, t22877, t22879, t22881, t22882) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1767::<F>(t22802, t22869, t553, t1338, t6955, t1352, t3851, t6987, t3856, t1372, t552, t1307);
    (t22870, t22871, t22873, t22874, t22877, t22879, t22881, t22882)
}
