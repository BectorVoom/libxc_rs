//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1134;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1135;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1136;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta320<F: Float>(t39378: F, t746: F, t9720: F, t1294: F, t1285: F, t9214: F, t12451: F, t1390: F, t12132: F, t588: F, t39253: F, t702: F, t9453: F, t12012: F, t12156: F, t12477: F, t1307: F, t1388: F, t193: F, t3719: F, t3918: F, t39529: F, t39531: F, t39533: F, t39539: F, t39541: F, t39549: F, t39563: F, t5126: F, t571: F, t2411: F, t2414: F, t701: F, t9777: F, t2405: F, t2415: F) -> (F, F, F, F, F, F, F, F) {
        let (t39568, t39570, t39572, t39577, t39582, t39585) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1134::<F>(t39378, t746, t9720, t1294, t1285, t9214, t12451, t1390, t12132, t588, t39253, t702, t9453);
        let t39586 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1135::<F>(t12012, t12156, t12477, t1307, t1388, t1390, t193, t3719, t3918, t39529, t39531, t39533, t39539, t39541, t39549, t39563, t39570, t39572, t39577, t39582, t39585, t5126, t571);
        let t39590 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1136::<F>(t2411, t2414, t701, t9777);
        let t39593 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1137::<F>(t2405, t2415, t9453);
    (t39568, t39570, t39572, t39582, t39585, t39586, t39590, t39593)
}
