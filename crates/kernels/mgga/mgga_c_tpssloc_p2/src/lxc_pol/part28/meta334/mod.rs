//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta334<F: Float>(t3726: F, t3770: F, t12211: F, t3766: F, t1358: F, t3774: F, t1333: F, t3862: F, t10022: F, t248: F, t557: F, t555: F) -> (F, F, F, F, F, F) {
        let (t12310, t12317, t12323, t12325, t12328, t12330) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1267::<F>(t3726, t3770, t12211, t3766, t1358, t3774, t1333, t3862, t10022, t248, t557, t555);
    (t12310, t12317, t12323, t12325, t12328, t12330)
}
