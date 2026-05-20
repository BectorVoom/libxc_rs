//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta300<F: Float>(t3103: F, t4641: F, t1040: F, t4616: F, t1612: F, t3082: F, t13969: F, t4584: F, t1041: F, t4589: F, t2960: F, t4603: F) -> (F, F, F, F, F, F, F, F) {
        let (t14084, t14085, t14117, t14134, t14136, t14137, t14139, t14158) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1465::<F>(t3103, t4641, t1040, t4616, t1612, t3082, t13969, t4584, t1041, t4589, t2960, t4603);
    (t14084, t14085, t14117, t14134, t14136, t14137, t14139, t14158)
}
