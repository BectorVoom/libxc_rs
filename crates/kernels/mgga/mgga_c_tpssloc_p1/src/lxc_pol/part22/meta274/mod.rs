//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta274<F: Float>(t12365: F, t1354: F, t1307: F, t3792: F, t3788: F, t835: F, t1336: F, t1995: F, t67: F, t246: F, t3777: F, t3802: F) -> (F, F, F, F, F, F, F) {
        let (t12366, t12369, t12384, t12385, t12418, t12419, t12429) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1423::<F>(t12365, t1354, t1307, t3792, t3788, t835, t1336, t1995, t67, t246, t3777, t3802);
    (t12366, t12369, t12384, t12385, t12418, t12419, t12429)
}
