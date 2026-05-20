//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta221<F: Float>(t1714: F, t4899: F, t11545: F, t60: F, t461: F, t11588: F, t134: F, t3439: F, t15026: F, t3032: F, t3514: F, t11147: F, t11778: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15390, t15394, t15395, t15402, t15418, t15419, t15437, t15438, t15453) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk869::<F>(t1714, t4899, t11545, t60, t461, t11588, t134, t3439, t15026, t3032, t3514, t11147, t11778);
    (t15390, t15394, t15395, t15402, t15418, t15419, t15437, t15438, t15453)
}
