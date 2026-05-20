//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk861;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta216<F: Float>(t13797: F, t344: F, t135: F, t340: F, t10189: F, t1597: F, t10224: F, t1592: F, t973: F, t1599: F, t698: F, t10508: F, t1616: F, t248: F, t1020: F, t122: F, t247: F) -> (F, F, F, F, F, F, F, F) {
        let (t13798, t13822, t13847, t13896, t13909, t13965) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk861::<F>(t13797, t344, t135, t340, t10189, t1597, t10224, t1592, t973, t1599, t698, t10508, t1616, t248);
        let (t13966, t13969) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk862::<F>(t1020, t13965, t122, t247);
    (t13798, t13822, t13847, t13896, t13909, t13965, t13966, t13969)
}
