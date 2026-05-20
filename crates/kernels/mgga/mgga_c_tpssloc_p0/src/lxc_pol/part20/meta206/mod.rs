//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1223;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1224;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1225;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1226;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1227;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1228;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1229;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta206<F: Float>(t1214: F, t248: F, t5012: F, t1017: F, t1742: F, t1210: F, t1207: F, t372: F, t479: F, t471: F, t1230: F, t4733: F, t3440: F, t4724: F, t1193: F, t1706: F, t135: F, t1725: F, t1174: F, t1196: F, t3966: F, t974: F, t1198: F, t1213: F, t1218: F, t1227: F, t1232: F, t1748: F, t3490: F, t3524: F, t3542: F, t3543: F, t3547: F, t3549: F, t3573: F, t4889: F, t5010: F, t466: F, t1752: F, t225: F, t1251: F, t1760: F, t3598: F, t1243: F, t5000: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5014, t5018, t5019) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1223::<F>(t1214, t248, t5012, t1017, t1742, t1210, t1207);
        let (t5023, t5024) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1224::<F>(t1742, t372, t479, t471);
        let (t5030, t5033, t5036, t5041, t5045) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1225::<F>(t1230, t248, t4733, t3440, t4724, t1193, t1706, t135, t1725, t1174, t1196, t3966);
        let t5051 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1226::<F>(t5045, t974, t1174, t1198, t1213, t1218, t1227, t1232, t1748, t3490, t3524, t3542, t3543, t3547, t3549, t3573, t4889, t5014, t5019, t5024, t5030, t5033, t5036, t5041);
        let t5052 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1227::<F>(t5010, t5051);
        let (t5053, t5055) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1228::<F>(t466, t5052, t1752, t225);
        let t5060 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1229::<F>(t1251, t1760, t3598);
        let t5064 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1230::<F>(t1243, t5000);
    (t5014, t5018, t5019, t5023, t5024, t5030, t5045, t5052, t5053, t5055, t5060, t5064)
}
