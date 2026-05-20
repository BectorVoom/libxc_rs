//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2447;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta651<F: Float>(t1015: F, t10472: F, t42559: F, t10870: F, t3048: F, t204: F, t376: F, t1020: F, t1023: F, t248: F, t10510: F, t3109: F, t3082: F, t3094: F, t1032: F, t10375: F, t370: F, t374: F, t9697: F, t10473: F, t361: F, t363: F, t42342: F, t42345: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43211, t43214, t43216, t43219, t43221) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2447::<F>(t1015, t10472, t42559, t10870, t3048, t204, t376, t1020, t1023, t248, t10510, t3109);
        let (t43228, t43248, t43253, t43288, t43291) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2448::<F>(t3082, t3094, t1032, t10375, t370, t374, t376, t9697, t10473, t361, t363, t42342, t42345);
    (t43211, t43214, t43216, t43219, t43221, t43228, t43248, t43253, t43288, t43291)
}
