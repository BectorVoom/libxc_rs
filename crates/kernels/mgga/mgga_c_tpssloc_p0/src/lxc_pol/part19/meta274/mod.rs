//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1035;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1036;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta274<F: Float>(t1285: F, t2221: F, t1287: F, t522: F, t9216: F, t9218: F, t1294: F, t9713: F, t25: F, t526: F, t3664: F, t606: F, t28: F, t11988: F, t2249: F, t514: F, t9257: F, t528: F, t1081: F, t3672: F, t11122: F, t12001: F, t3231: F, t517: F, zeta_threshold: F, t157: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12051, t12053, t12055, t12057, t12059, t12061, t12064) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1035::<F>(t1285, t2221, t1287, t522, t9216, t9218, t1294, t9713, t25, t526, t3664, t606);
        let (t12070, t12072, t12075, t12081) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1036::<F>(t25, t28, t11988, t12061, t12064, t2249, t514, t9257, t528, t1081, t3672, t11122, t12001, t3231, t517, zeta_threshold);
        let t12083 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1037::<F>(t12070, t12081, t157);
    (t12051, t12053, t12055, t12057, t12059, t12061, t12064, t12072, t12075, t12083)
}
