//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk842;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta161<F: Float>(t2768: F, t4338: F, t123: F, t1409: F, t2775: F, t607: F, t882: F, t3966: F, t883: F) -> (F, F, F, F, F, F, F) {
        let (t4339, t4340, t4342, t4343) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk842::<F>(t2768, t4338, t123, t1409, t2775, t607);
        let (t4344, t4345, t4347) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk843::<F>(t4343, t882, t123, t3966, t883);
    (t4339, t4340, t4342, t4343, t4344, t4345, t4347)
}
