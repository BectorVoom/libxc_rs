//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1844;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta493<F: Float>(t3252: F, t7286: F, t7285: F, t3248: F, t24574: F, t7288: F, t225: F, t7306: F, t2154: F, t3599: F, t11606: F, t11925: F, t11928: F, t1238: F, t1252: F, t2155: F, t24630: F, t24634: F, t24639: F, t24646: F, t24758: F, t24868: F, t24871: F, t24873: F, t24877: F, t24880: F, t3593: F, t3631: F, t498: F, t7283: F, t7351: F, t7392: F, t265: F, t504: F, t24629: F, t3640: F, t7394: F, t11947: F, t2157: F, t1254: F, t1256: F, t193: F, t23772: F, t336: F, t3633: F, t3637: F, t4700: F, t7398: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24883, t24884, t24887, t24888, t24891, t24893, t24897, t24900) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1844::<F>(t3252, t7286, t7285, t3248, t24574, t7288, t225, t7306, t2154, t3599, t11606, t11925, t11928, t1238, t1252, t2155, t24630, t24634, t24639, t24646, t24758, t24868, t24871, t24873, t24877, t24880, t3593, t3631, t498, t7283, t7351, t7392);
        let (t24901, t24905, t24909, t24916) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1845::<F>(t265, t504, t24629, t24900, t3640, t7394, t11947, t2157, t1254, t1256, t193, t23772, t336, t3633, t3637, t4700, t7398);
    (t24883, t24884, t24887, t24888, t24891, t24893, t24897, t24901, t24905, t24909, t24916)
}
