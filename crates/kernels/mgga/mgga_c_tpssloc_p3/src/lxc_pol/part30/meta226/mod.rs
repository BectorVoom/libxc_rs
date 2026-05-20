//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1049;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1050;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta226<F: Float>(t3240: F, t5971: F, t123: F, t3247: F, t5392: F, t1088: F, t1089: F, t5398: F) -> (F, F, F, F, F, F) {
        let (t5972, t5973, t5975) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1049::<F>(t3240, t5971, t123, t3247, t5392);
        let (t5976, t5977, t5979) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1050::<F>(t1088, t5975, t123, t1089, t5398);
    (t5972, t5973, t5975, t5976, t5977, t5979)
}
