//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1029;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1030;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1031;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta222<F: Float>(t360: F, t5866: F, t1021: F, t248: F, t1615: F, t3131: F) -> (F, F, F, F, F, F, F) {
        let (t5867, t5869) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1029::<F>(t360, t5866, t1021, t248);
        let t5872 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1030::<F>(t1615);
        let (t5873, t5875) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1031::<F>(t3131, t5872, t1021, t248);
        let (t5878, t5880) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1032::<F>(t360, t5872, t1021, t248);
    (t5867, t5869, t5872, t5873, t5875, t5878, t5880)
}
