//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta310<F: Float>(t21753: F, t21808: F, t1118: F, t1099: F, t11277: F, t21723: F, t11275: F, t11136: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F) -> (F, F, F, F, F, F) {
        let (t21809, t21810, t21812, t21813, t21815, t21826) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1060::<F>(t21753, t21808, t1118, t1099, t11277, t21723, t11275, t11136, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778);
    (t21809, t21810, t21812, t21813, t21815, t21826)
}
