//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2110;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta555<F: Float>(t2289: F, t2769: F, t10537: F, t690: F, t41654: F, t10603: F, t2932: F, t2784: F, t2791: F, t2897: F, t2929: F, t10629: F, t938: F, t2903: F, t2928: F, t315: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41687, t41713) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2110::<F>(t2289, t2769, t10537, t690);
        let (t41741, t41769, t41811, t41816, t41821, t41825, t41826) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2111::<F>(t41654, t10603, t2932, t2784, t2791, t2897, t2929, t10629, t938, t2903, t2928, t315);
    (t41687, t41713, t41741, t41769, t41811, t41816, t41821, t41825, t41826)
}
