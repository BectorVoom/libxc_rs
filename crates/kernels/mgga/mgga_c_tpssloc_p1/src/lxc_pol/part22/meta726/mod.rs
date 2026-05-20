//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta726 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2379;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta726<F: Float>(t48140: F, t48143: F, t68513: F, t42444: F, t20234: F, t41687: F, t607: F, t10304: F, t136: F, t17151: F, t3966: F, t41880: F, t68477: F, t68498: F, t68500: F, t68502: F, t68504: F, t68506: F, t68509: F, t68511: F) -> (F, F, F, F, F, F, F, F) {
        let (t68515, t68518, t68521, t68523, t68525, t68527, t68530) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2379::<F>(t48140, t48143, t68513, t42444, t20234, t41687, t607, t10304, t136, t17151, t3966, t41880, t68477);
        let t68532 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2380::<F>(t68498, t68500, t68502, t68504, t68506, t68509, t68511, t68515, t68518, t68523, t68527, t68530);
    (t68515, t68518, t68521, t68523, t68525, t68527, t68530, t68532)
}
