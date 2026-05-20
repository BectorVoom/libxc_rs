//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk222;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk223;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta29<F: Float>(t576: F, t577: F, t11: F, t2: F, t10: F, t3: F, t9: F, t16: F, t15: F, t14: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t580, t581, t582, t583, t584, t586, t587, t588) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk222::<F>(t576, t577, t11, t2, t10, t3, t9, t16);
        let (t589, t590, t591) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk223::<F>(t588, t15, t3);
        let t592 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk224::<F>(t14, t591);
    (t580, t581, t582, t583, t584, t586, t587, t588, t589, t590, t591, t592)
}
