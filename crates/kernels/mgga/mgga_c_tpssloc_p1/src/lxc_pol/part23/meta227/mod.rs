//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk875;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta227<F: Float>(t16046: F, t544: F, t12189: F, t1804: F, t5194: F, t782: F, t3732: F, t67: F, t792: F, t1799: F, t212: F, t12214: F, t131: F, t205: F, t12199: F, t5202: F, t12225: F, t2586: F, t2371: F, t5154: F, t12365: F, t1827: F, t12418: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16047, t16078, t16081, t16094, t16095, t16100) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk875::<F>(t16046, t544, t12189, t1804, t5194, t782, t3732, t67, t792, t1799, t212, t12214, t131);
        let (t16101, t16108, t16118, t16119, t16164, t16211, t16224) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk876::<F>(t16100, t205, t12199, t5202, t12225, t16095, t2586, t2371, t5154, t12365, t1827, t12418, t820);
    (t16047, t16078, t16081, t16094, t16095, t16101, t16108, t16118, t16119, t16164, t16211, t16224)
}
