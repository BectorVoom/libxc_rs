//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta323<F: Float>(t10471: F, t3502: F, t11712: F, t3508: F, t6739: F, t11707: F, t3609: F, t3623: F, t1209: F, t225: F, t3591: F, t3482: F) -> (F, F, F, F, F, F, F) {
        let (t11888, t11889, t11904, t11907, t11914, t11925, t11928) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1254::<F>(t10471, t3502, t11712, t3508, t6739, t11707, t3609, t3623, t1209, t225, t3591, t3482);
    (t11888, t11889, t11904, t11907, t11914, t11925, t11928)
}
