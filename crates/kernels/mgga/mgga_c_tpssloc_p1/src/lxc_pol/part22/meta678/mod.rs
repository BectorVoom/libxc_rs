//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2239;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta678<F: Float>(t17659: F, t3117: F, t1041: F, t17187: F, t248: F, t3051: F, t10422: F, t17704: F, t3070: F, t17680: F, t13969: F, t17692: F, t14077: F, t4630: F, t10482: F, t5872: F, t10413: F, t17924: F, t17959: F, t376: F, t10480: F, t17672: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t61977, t61981, t62013, t62032, t62038) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2239::<F>(t17659, t3117, t1041, t17187, t248, t3051, t10422, t17704, t3070, t17680, t13969, t17692);
        let (t62049, t62079, t62085, t62091, t62099) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2240::<F>(t14077, t4630, t10482, t5872, t10413, t10422, t17924, t17959, t376, t10480, t13969, t17672);
    (t61977, t61981, t62013, t62032, t62038, t62049, t62079, t62085, t62091, t62099)
}
