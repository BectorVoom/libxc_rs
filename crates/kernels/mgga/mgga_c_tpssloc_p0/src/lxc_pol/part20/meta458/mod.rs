//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta458<F: Float>(t1714: F, t4899: F, t11571: F, t11545: F, t60: F, t461: F, t14726: F, t11589: F, t4904: F, t3447: F, t11588: F) -> (F, F, F, F, F, F, F) {
        let (t15390, t15391, t15394, t15395, t15396, t15401, t15402) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1919::<F>(t1714, t4899, t11571, t11545, t60, t461, t14726, t11589, t4904, t3447, t11588);
    (t15390, t15391, t15394, t15395, t15396, t15401, t15402)
}
