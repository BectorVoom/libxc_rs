//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk846;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta151<F: Float>(t3243: F, t3297: F, t136: F, t1113: F, t3248: F, t3252: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3282: F, t3288: F, t3290: F, t3294: F, t3295: F, t1118: F, t1099: F, t1097: F, t409: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3298, t3299, t3301, t3302, t3304, t3305, t3307) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk846::<F>(t3243, t3297, t136, t1113, t3248, t3252, t3238, t3245, t3250, t3254, t3272, t3280, t3282, t3288, t3290, t3294, t3295);
        let (t3308, t3310, t3311, t3312, t3313) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk847::<F>(t1118, t3307, t1099, t1097, t409);
    (t3298, t3299, t3301, t3302, t3304, t3305, t3307, t3308, t3310, t3311, t3312, t3313)
}
