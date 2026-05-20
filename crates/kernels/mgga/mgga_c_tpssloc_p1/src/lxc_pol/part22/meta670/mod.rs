//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta670<F: Float>(t17579: F, t225: F, t18048: F, t17826: F, t2960: F, t10236: F, t17686: F, t43070: F, t10254: F, t17635: F, t17691: F, t135: F, t17843: F, t973: F) -> (F, F, F, F, F, F, F, F) {
        let (t61058, t61061, t61074, t61082, t61086, t61094, t61103, t61172) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2225::<F>(t17579, t225, t18048, t17826, t2960, t10236, t17686, t43070, t10254, t17635, t17691, t135, t17843, t973);
    (t61058, t61061, t61074, t61082, t61086, t61094, t61103, t61172)
}
