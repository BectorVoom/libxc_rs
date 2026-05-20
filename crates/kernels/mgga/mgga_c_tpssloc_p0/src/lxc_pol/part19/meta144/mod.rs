//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta144<F: Float>(t3879: F, t539: F, t1373: F, t225: F, t1376: F, t566: F, t68: F, t1385: F, t3787: F, t562: F, t3793: F, t1338: F, t1372: F) -> (F, F, F, F, F, F, F, F) {
        let (t3880, t3882, t3887, t3888, t3889, t3897, t3898, t3901) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk744::<F>(t3879, t539, t1373, t225, t1376, t566, t68, t1385, t3787, t562, t3793, t1338, t1372);
    (t3880, t3882, t3887, t3888, t3889, t3897, t3898, t3901)
}
