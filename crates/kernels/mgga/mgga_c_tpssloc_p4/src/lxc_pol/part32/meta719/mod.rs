//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta719 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2285;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta719<F: Float>(t55388: F, t7015: F, t20173: F, t28896: F, t28893: F, t6534: F, t1401: F, t96729: F, t16524: F, t26542: F, t1458: F, t26135: F, t3941: F, t4072: F, t7467: F, t28017: F, t3938: F, t12524: F, t28899: F, t75795: F, t7769: F, t5371: F, t26550: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t100875, t100879, t100883, t100885, t100887, t100890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2285::<F>(t55388, t7015, t20173, t28896, t28893, t6534, t1401, t96729, t16524, t26542, t1458, t26135, t3941);
        let (t100893, t100897, t100899, t100902, t100908, t100915) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2286::<F>(t3941, t4072, t7467, t28017, t3938, t12524, t28899, t75795, t7769, t26135, t5371, t16524, t26550);
    (t100875, t100879, t100883, t100885, t100887, t100890, t100893, t100897, t100899, t100902, t100908, t100915)
}
