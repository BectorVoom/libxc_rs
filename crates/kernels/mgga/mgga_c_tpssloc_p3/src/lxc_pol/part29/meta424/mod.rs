//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1711;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1712;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta424<F: Float>(t2632: F, t4233: F, t1441: F, t671: F, t1388: F, t1799: F, t3792: F, t5286: F, t576: F, t1874: F, t9348: F, t4034: F, t6535: F, t107: F, t240: F, t109: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t2332: F, t2358: F, t6530: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16935, t19456) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1711::<F>(t2632, t4233, t1441, t671);
        let (t19577, t19735, t20173, t22460, t22467, t22468) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1712::<F>(t1388, t1799, t3792, t5286, t576, t671, t1874, t9348, t4034, t6535, t107, t240);
        let (t22469, t22470, t22471, t22473, t22479) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1713::<F>(t109, t22468, t625, t656, t666, t2331, t63, t2332, t2358, t6530);
    (t16935, t19456, t19577, t19735, t20173, t22460, t22467, t22469, t22470, t22471, t22473, t22479)
}
