//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta596<F: Float>(t11521: F, t1174: F, t3431: F, t1184: F, t15394: F, t11147: F, t460: F, t9288: F, t11588: F, t3469: F, t3447: F, t3451: F) -> (F, F, F, F, F, F) {
        let (t44502, t44504, t44505, t44506, t44510, t44512) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2176::<F>(t11521, t1174, t3431, t1184, t15394, t11147, t460, t9288, t11588, t3469, t3447, t3451);
    (t44502, t44504, t44505, t44506, t44510, t44512)
}
