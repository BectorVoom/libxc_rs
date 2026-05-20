//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk895;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk896;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta170<F: Float>(t2375: F, t3684: F, t1294: F, t2371: F, t2528: F, t1284: F, t172: F, t763: F, t2535: F, t184: F, t3681: F, t17: F, t1388: F, t570: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3686, t3688, t3690, t3691, t3692, t3693, t3695, t3696, t3697) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk895::<F>(t2375, t3684, t1294, t2371, t2528, t1284, t172, t763, t2535, t184, t3681, t17);
        let t3698 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk896::<F>(t1388);
        let (t3700, t3701) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk897::<F>(t570);
    (t3686, t3688, t3690, t3691, t3692, t3693, t3695, t3696, t3697, t3698, t3700, t3701)
}
