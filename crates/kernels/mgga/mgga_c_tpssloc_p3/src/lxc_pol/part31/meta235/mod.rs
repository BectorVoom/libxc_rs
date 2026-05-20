//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk982;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk983;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta235<F: Float>(t475: F, t6218: F, t1214: F, t248: F, t1734: F, t3508: F, t1213: F, t1227: F, t1737: F, t1748: F, t3506: F, t3515: F, t3542: F, t3547: F, t467: F, t5005: F, t5019: F, t5024: F, t5036: F, t5041: F, t6109: F, t6203: F, t6207: F, t6211: F, t6197: F, t466: F, t1760: F, t3598: F, t491: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6219, t6221, t6224) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk982::<F>(t475, t6218, t1214, t248, t1734);
        let (t6225, t6227, t6230, t6232, t6237) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk983::<F>(t3508, t6224, t1214, t248, t475, t1213, t1227, t1737, t1748, t3506, t3515, t3542, t3547, t467, t5005, t5019, t5024, t5036, t5041, t6109, t6203, t6207, t6211, t6221);
        let (t6238, t6239, t6243, t6244, t6252) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk984::<F>(t6197, t6237, t466, t1760, t3598, t491, t6224);
    (t6219, t6221, t6224, t6225, t6227, t6230, t6232, t6238, t6239, t6243, t6244, t6252)
}
