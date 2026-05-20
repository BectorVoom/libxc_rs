//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta252<F: Float>(t135: F, t3471: F, t1174: F, t11168: F, t4908: F, t11159: F, t4900: F, t1184: F, t4899: F, t3242: F, t460: F, t2244: F) -> (F, F, F, F, F, F) {
        let (t11561, t11563, t11566, t11569, t11570, t11571) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk996::<F>(t135, t3471, t1174, t11168, t4908, t11159, t4900, t1184, t4899, t3242, t460, t2244);
    (t11561, t11563, t11566, t11569, t11570, t11571)
}
