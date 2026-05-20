//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1719;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta416<F: Float>(t2632: F, t4233: F, t1441: F, t671: F, t1388: F, t1799: F, t3792: F, t5286: F, t576: F, t1874: F, t9348: F, t111: F, t6514: F) -> (F, F, F, F, F, F, F) {
        let (t16935, t19456) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1719::<F>(t2632, t4233, t1441, t671);
        let (t19577, t19735, t20173, t22460, t22461) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1720::<F>(t1388, t1799, t3792, t5286, t576, t671, t1874, t9348, t111, t6514);
    (t16935, t19456, t19577, t19735, t20173, t22460, t22461)
}
