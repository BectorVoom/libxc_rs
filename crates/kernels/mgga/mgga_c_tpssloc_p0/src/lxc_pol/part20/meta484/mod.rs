//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta484<F: Float>(t5194: F, t782: F, t5198: F, t213: F, t5187: F, t1307: F, t221: F, t3719: F, t5196: F, t3732: F, t67: F, t792: F) -> (F, F, F, F, F, F, F) {
        let (t16081, t16083, t16084, t16086, t16090, t16093, t16094) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1970::<F>(t5194, t782, t5198, t213, t5187, t1307, t221, t3719, t5196, t3732, t67, t792);
    (t16081, t16083, t16084, t16086, t16090, t16093, t16094)
}
