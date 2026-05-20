//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta59 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk428;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk429;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk430;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk431;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk432;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta59<F: Float>(t1096: F, t1121: F, t1124: F, t1129: F, t1138: F, t1144: F, t1148: F, t1157: F, t300: F, t436: F, t440: F, t1147: F, t1155: F, t1156: F, t134: F, t457: F, t461: F, t221: F, t456: F, t51: F, t972: F, t404: F, t405: F, t974: F, t1089: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1161, t1163, t1164) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk428::<F>(t1096, t1121, t1124, t1129, t1138, t1144, t1148, t1157, t300, t436, t440);
        let t1166 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk429::<F>(t1147, t1155, t1156);
        let (t1168, t1169, t1171, t1173, t1174) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk430::<F>(t1164, t1166, t134, t457, t461, t221, t456, t51, t972);
        let t1176 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk431::<F>(t404, t405);
        let t1177 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk432::<F>(t1176, t974);
        let t1178 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk433::<F>(t1089, t461);
    (t1161, t1163, t1164, t1166, t1168, t1169, t1171, t1173, t1174, t1176, t1177, t1178)
}
