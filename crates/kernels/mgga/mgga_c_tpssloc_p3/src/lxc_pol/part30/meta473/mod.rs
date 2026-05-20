//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1767;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta473<F: Float>(t1049: F, t362: F, t225: F, t23592: F, t23384: F, t6787: F, t3216: F, t6818: F, t11094: F, t1958: F, t2752: F, t28: F, t112: F, t7002: F, t111: F, t2022: F, t1976: F, t4072: F, t671: F, t7670: F, t191: F, t192: F, t5118: F, t2020: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23685, t23696, t23712, t23738, t23742, t23788) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1767::<F>(t1049, t362, t225, t23592, t23384, t6787, t3216, t6818, t11094, t1958, t2752, t28);
        let (t23877, t23880, t24980, t24983, t24987, t24988) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1768::<F>(t112, t7002, t111, t2022, t1976, t4072, t671, t7670, t191, t192, t5118, t2020);
    (t23685, t23696, t23712, t23738, t23742, t23788, t23877, t23880, t24980, t24983, t24987, t24988)
}
