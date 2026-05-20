//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1772;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta357<F: Float>(t2639: F, t4236: F, t1512: F, t9674: F, t2638: F, t4166: F, t831: F, t2629: F, t4250: F, t9638: F, t1495: F, t210: F, t2379: F, t4158: F, t776: F, t2553: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13275, t13277, t13278) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1772::<F>(t2639, t4236, t1512, t9674, t2638, t4166);
        let (t13280, t13283, t13287, t13289, t13293, t13297) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1773::<F>(t13278, t831, t2629, t4166, t4250, t9638, t1495, t210, t2379, t4158, t776, t2553);
    (t13275, t13277, t13278, t13280, t13283, t13287, t13289, t13293, t13297)
}
