//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2563;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta679<F: Float>(t11126: F, t4875: F, t14858: F, t3415: F, t11294: F, t4869: F, t15044: F, t3411: F, t11300: F, t1164: F, t14841: F, t3419: F, t3423: F, t51839: F, t51844: F, t51847: F, t51851: F) -> (F, F, F, F, F, F, F, F) {
        let (t51853, t51855, t51857, t51859, t51862, t51864) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2563::<F>(t11126, t4875, t14858, t3415, t11294, t4869, t15044, t3411, t11300, t1164, t14841, t3419);
        let (t51866, t51867) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2564::<F>(t14858, t3423, t51839, t51844, t51847, t51851, t51853, t51855, t51857, t51859, t51862, t51864);
    (t51853, t51855, t51857, t51859, t51862, t51864, t51866, t51867)
}
