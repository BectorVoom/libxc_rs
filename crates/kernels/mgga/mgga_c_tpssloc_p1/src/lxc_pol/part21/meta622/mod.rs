//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2400;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta622<F: Float>(t212: F, t2586: F, t3734: F, t40353: F, t12225: F, t3719: F, t116: F, t1314: F, t9534: F, t1307: F, t133: F, t6600: F, t3736: F, t40018: F, t59: F, t9223: F, t120: F, t22815: F, t67: F, t535: F, t1317: F, t40005: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40356, t40360, t40369, t40372) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2400::<F>(t212, t2586, t3734, t40353, t12225, t3719, t116, t1314, t9534, t1307, t133, t6600);
        let (t40387, t40394, t40399, t40401, t40402) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2401::<F>(t3736, t40018, t59, t9223, t116, t120, t212, t22815, t67, t535, t1317, t40005);
    (t40356, t40360, t40369, t40372, t40387, t40394, t40399, t40401, t40402)
}
