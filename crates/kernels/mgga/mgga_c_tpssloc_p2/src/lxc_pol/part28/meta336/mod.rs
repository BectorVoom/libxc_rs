//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta336<F: Float>(t3866: F, t3872: F, t3876: F, t1339: F, t2690: F, t1336: F, t1354: F, t1307: F, t3792: F, t3788: F, t835: F, t3795: F) -> (F, F, F, F, F, F) {
        let (t12356, t12358, t12365, t12366, t12369, t12386) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1269::<F>(t3866, t3872, t3876, t1339, t2690, t1336, t1354, t1307, t3792, t3788, t835, t3795);
    (t12356, t12358, t12365, t12366, t12369, t12386)
}
