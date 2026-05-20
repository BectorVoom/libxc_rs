//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1440;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta287<F: Float>(t2617: F, t4177: F, t2628: F, t836: F, t812: F, t4184: F, t242: F, t9972: F, t2639: F, t4236: F, t1512: F, t9674: F, t2638: F, t4166: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13254, t13257, t13258) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1440::<F>(t2617, t4177, t2628, t836, t812);
        let (t13260, t13261, t13262, t13275, t13277, t13278) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1441::<F>(t13258, t4184, t242, t9972, t812, t2639, t4236, t1512, t9674, t2638, t4166);
    (t13254, t13257, t13258, t13260, t13261, t13262, t13275, t13277, t13278)
}
