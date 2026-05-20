//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1938;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta569<F: Float>(t343: F, t5836: F, t6734: F, t5842: F, t1941: F, t5904: F, t1011: F, t5872: F, t3131: F, t23512: F, t360: F, t23519: F, t5866: F, t68: F, t6744: F, t1935: F, t23419: F, t23469: F, t23510: F, t25639: F, t25642: F, t25683: F, t378: F, t5885: F, t5890: F, t5894: F, t5900: F, t5909: F, t6717: F, t6742: F, t6765: F, t7574: F, t7578: F, t7583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28557, t28558, t28565, t28566, t28572, t28577, t28578, t28581, t28582) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1938::<F>(t343, t5836, t6734, t5842, t1941, t5904, t1011, t5872, t3131, t23512, t360, t23519);
        let (t28586, t28587, t28592) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1939::<F>(t360, t5866, t68, t6744, t1935, t23419, t23469, t23510, t25639, t25642, t25683, t28558, t28566, t28572, t28578, t28582, t378, t5885, t5890, t5894, t5900, t5909, t6717, t6742, t6765, t7574, t7578, t7583);
    (t28557, t28558, t28565, t28566, t28572, t28577, t28578, t28581, t28582, t28586, t28587, t28592)
}
