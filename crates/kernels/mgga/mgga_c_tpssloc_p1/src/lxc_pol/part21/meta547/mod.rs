//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta547<F: Float>(t136: F, t18499: F, t18215: F, t3297: F, t6014: F, t699: F, t1113: F, t18221: F, t18225: F, t6017: F, t18232: F, t18237: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18500, t18502, t18503, t18505, t18507, t18508, t18509, t18510, t18512, t18514, t18515, t18517) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2237::<F>(t136, t18499, t18215, t3297, t6014, t699, t1113, t18221, t18225, t6017, t18232, t18237);
    (t18500, t18502, t18503, t18505, t18507, t18508, t18509, t18510, t18512, t18514, t18515, t18517)
}
