//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk498;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk499;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk500;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk501;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta79<F: Float>(t1557: F, t893: F, t1541: F, t917: F, t1548: F, t1551: F, t1554: F, t926: F, t929: F, t932: F, t936: F, t324: F, t945: F, t948: F, t951: F, t1545: F, t300: F, t311: F, t924: F, t943: F, t942: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1559, t1561, t1568, t1569) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk498::<F>(t1557, t893, t1541, t917, t1548, t1551, t1554, t926, t929, t932);
        let t1573 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk499::<F>(t1541, t936);
        let (t1574, t1580) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk500::<F>(t1573, t324, t1541, t1548, t1551, t1554, t945, t948);
        let t1581 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk501::<F>(t1580, t951);
        let (t1585, t1587, t1589) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk502::<F>(t1545, t1559, t1561, t1569, t1574, t1581, t300, t311, t924, t943, t1580, t942, t951);
    (t1559, t1561, t1568, t1569, t1573, t1580, t1581, t1585, t1587, t1589)
}
