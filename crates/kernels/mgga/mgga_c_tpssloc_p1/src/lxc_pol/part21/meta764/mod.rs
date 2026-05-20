//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta764 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2640;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta764<F: Float>(t2559: F, t5194: F, t5198: F, t118: F, t16018: F, t3739: F, t794: F, t16081: F, t16086: F, t12214: F, t67: F, t792: F, t16095: F, t3734: F, t686: F, t133: F, t1799: F, t40369: F, t6600: F, t131: F, t205: F, t40024: F, t1336: F, t242: F, t40042: F) -> (F, F, F, F, F, F, F, F) {
        let (t54701, t54705, t54711, t54718) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2640::<F>(t2559, t5194, t5198, t118, t16018, t3739, t794, t16081, t16086, t12214, t67, t792);
        let (t54721, t54725, t54728, t54744) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2641::<F>(t16095, t3734, t54718, t686, t133, t1799, t40369, t6600, t131, t205, t40024, t1336, t242, t40042);
    (t54701, t54705, t54711, t54718, t54721, t54725, t54728, t54744)
}
