//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1325;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta239<F: Float>(t2617: F, t2696: F, t2693: F, t809: F, t597: F, t61: F, t241: F, t244: F, t248: F, t238: F, t154: F, t9569: F, t222: F, t805: F, t9541: F, t2627: F, t852: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9993, t10014, t10021) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1325::<F>(t2617, t2696, t2693, t809, t597, t61);
        let (t10022, t10024, t10026, t10027, t10029, t10036, t10054) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1326::<F>(t10021, t241, t244, t248, t238, t154, t9569, t222, t805, t9541, t2627, t852);
    (t9993, t10014, t10021, t10022, t10024, t10026, t10027, t10029, t10036, t10054)
}
