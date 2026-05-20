//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta518<F: Float>(t3131: F, t4649: F, t4593: F, t4582: F, t16558: F, t998: F, t974: F, t13835: F, t4531: F, t13769: F, t13839: F, t1539: F, t6733: F) -> (F, F, F, F, F, F, F, F) {
        let (t17732, t17733, t17734, t17737, t17738, t17742, t17745, t17748) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2169::<F>(t3131, t4649, t4593, t4582, t16558, t998, t974, t13835, t4531, t13769, t13839, t1539, t6733);
    (t17732, t17733, t17734, t17737, t17738, t17742, t17745, t17748)
}
