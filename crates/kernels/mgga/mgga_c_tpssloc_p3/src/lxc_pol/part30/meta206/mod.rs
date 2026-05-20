//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk975;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk976;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta206<F: Float>(t5: F, t1437: F, t2240: F, t3953: F, t5385: F, t5389: F, t5445: F, t605: F, t86: F, t112: F, t1458: F, t89: F, t1774: F, t1453: F) -> (F, F, F, F, F, F) {
        let (t5449, t5450) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk975::<F>(t5, t1437, t2240, t3953, t5385, t5389, t5445, t605, t86, t112);
        let t5456 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk976::<F>(t1458);
        let (t5457, t5460, t5464) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk977::<F>(t5456, t89, t1458, t1774, t1453);
    (t5449, t5450, t5456, t5457, t5460, t5464)
}
