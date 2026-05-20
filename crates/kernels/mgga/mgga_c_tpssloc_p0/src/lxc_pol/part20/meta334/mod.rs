//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1623;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta334<F: Float>(t1176: F, t3242: F, t9288: F, t974: F, t11638: F, t475: F, t1214: F, t248: F, t11616: F, t68: F, t484: F, t10913: F, t4972: F, t4582: F, t1174: F, t11821: F, t11825: F, t11834: F, t11836: F, t11839: F, t11842: F, t11845: F, t1213: F, t1227: F, t1232: F, t3490: F, t3527: F, t3531: F, t3587: F, t488: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11849, t11850, t11853, t11855, t11858, t11859, t11862) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1623::<F>(t1176, t3242, t9288, t974, t11638, t475, t1214, t248, t11616, t68, t484, t10913, t4972);
        let (t11863, t11866) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1624::<F>(t11862, t4582, t1174, t11821, t11825, t11834, t11836, t11839, t11842, t11845, t11850, t11855, t11859, t1213, t1227, t1232, t3490, t3527, t3531, t3587, t488);
    (t11849, t11850, t11853, t11855, t11858, t11859, t11862, t11863, t11866)
}
