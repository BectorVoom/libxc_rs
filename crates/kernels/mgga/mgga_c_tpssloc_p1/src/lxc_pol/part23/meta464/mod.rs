//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1358;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta464<F: Float>(t21101: F, t4483: F, t5726: F, t2842: F, t2844: F, t21373: F, t10702: F, t5694: F, t60378: F, t17492: F, t17947: F, t959: F, t4475: F, t68902: F, t17934: F, t5812: F, t21370: F, t76665: F, t76668: F, t76671: F, t76674: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t76997, t76998, t77001, t77003, t77006, t77009) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1358::<F>(t21101, t4483, t5726, t2842, t2844, t21373, t10702, t5694, t60378, t17492, t17947, t959);
        let (t77012, t77014, t77016, t77017) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1359::<F>(t4475, t68902, t959, t17934, t5812, t21370, t4483, t76665, t76668, t76671, t76674, t76997, t77001, t77003, t77006, t77009);
    (t76997, t76998, t77001, t77003, t77006, t77009, t77012, t77014, t77016, t77017)
}
